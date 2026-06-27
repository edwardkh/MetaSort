use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Mutex;
use chrono::{TimeZone, Utc};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::csv_report;
use crate::utils::log_to_file;
use serde_json;
use crate::platform::get_exiftool_command;
use std::collections::{HashSet, HashMap};

pub struct SortCounts {
    pub photos: usize,
    pub videos: usize,
    pub whatsapp: usize,
    pub screenshots: usize,
    pub unknown: usize,
    pub mkv: usize,
}

type FileInfo = (String, String, String, String, String, u64);

struct SortState {
    photos_info: Vec<FileInfo>,
    videos_info: Vec<FileInfo>,
    unknown_info: Vec<FileInfo>,
    mkv_info: Vec<FileInfo>,
    failed_guess_info: Vec<FileInfo>,
    counts: SortCounts,
}

impl SortState {
    fn new() -> Self {
        Self {
            photos_info: Vec::new(),
            videos_info: Vec::new(),
            unknown_info: Vec::new(),
            mkv_info: Vec::new(),
            failed_guess_info: Vec::new(),
            counts: SortCounts { photos: 0, videos: 0, whatsapp: 0, screenshots: 0, unknown: 0, mkv: 0 },
        }
    }
    
    // Helper to safely merge states from threads
    fn merge(&mut self, other: SortState) {
        self.photos_info.extend(other.photos_info);
        self.videos_info.extend(other.videos_info);
        self.unknown_info.extend(other.unknown_info);
        self.mkv_info.extend(other.mkv_info);
        self.failed_guess_info.extend(other.failed_guess_info);
        
        self.counts.photos += other.counts.photos;
        self.counts.videos += other.counts.videos;
        self.counts.whatsapp += other.counts.whatsapp;
        self.counts.screenshots += other.counts.screenshots;
        self.counts.unknown += other.counts.unknown;
        self.counts.mkv += other.counts.mkv;
    }
}

fn get_unique_dest_path(
    source_path: &Path,
    dest_dir: &Path,
    claimed_paths: &Mutex<HashSet<PathBuf>>,
    live_photo_map: &Mutex<HashMap<PathBuf, String>>,
) -> PathBuf {
    let stem = source_path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = source_path.extension().and_then(|e| e.to_str());
    let ext_lower = ext.unwrap_or("").to_lowercase();

    // Identify if this is a component of an iOS Live Photo
    let is_live = if ext_lower == "heic" {
        source_path.with_extension("mp4").exists() || source_path.with_extension("MP4").exists()
    } else if ext_lower == "mp4" {
        source_path.with_extension("heic").exists() || source_path.with_extension("HEIC").exists()
    } else {
        false
    };

    // The key uniquely identifies the pair based on its original source location
    let live_key = if is_live {
        Some(source_path.with_extension("")) 
    } else {
        None
    };

    let mut locked_live_map = live_photo_map.lock().unwrap();
    let mut locked_claimed = claimed_paths.lock().unwrap();

    // If the other half of this Live Photo was already processed, use its assigned stem
    if let Some(key) = &live_key {
        if let Some(assigned_stem) = locked_live_map.get(key) {
            let final_name = match ext {
                Some(e) => format!("{}.{}", assigned_stem, e),
                None => assigned_stem.to_string(),
            };
            let full_dest = dest_dir.join(&final_name);
            locked_claimed.insert(full_dest.clone());
            return full_dest;
        }
    }

    let mut counter = 1;
    let mut candidate_stem = stem.to_string();
    
    loop {
        let mut is_free = true;
        
        if is_live {
            // For live photos, BOTH extensions must be free in the destination to prevent splitting the pair
            let heic_ext = if ext_lower == "heic" { ext.unwrap() } else { "HEIC" };
            let mp4_ext = if ext_lower == "mp4" { ext.unwrap() } else { "MP4" };
            
            let heic_dest = dest_dir.join(format!("{}.{}", candidate_stem, heic_ext));
            let mp4_dest = dest_dir.join(format!("{}.{}", candidate_stem, mp4_ext));
            
            if heic_dest.exists() || locked_claimed.contains(&heic_dest) || 
               mp4_dest.exists() || locked_claimed.contains(&mp4_dest) {
                is_free = false;
            }
        } else {
            let candidate_name = match ext {
                Some(e) => format!("{}.{}", candidate_stem, e),
                None => candidate_stem.clone(),
            };
            let full_dest = dest_dir.join(&candidate_name);
            if full_dest.exists() || locked_claimed.contains(&full_dest) {
                is_free = false;
            }
        }

        if is_free {
            // If this is a Live Photo, remember the successful stem for the counterpart
            if let Some(key) = live_key {
                locked_live_map.insert(key, candidate_stem.clone());
            }
            
            let final_name = match ext {
                Some(e) => format!("{}.{}", candidate_stem, e),
                None => candidate_stem,
            };
            let full_dest = dest_dir.join(&final_name);
            locked_claimed.insert(full_dest.clone());
            return full_dest;
        }

        candidate_stem = format!("{}_{}", stem, counter);
        counter += 1;
    }
}

fn human_readable_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    match size {
        s if s >= GB => format!("{:.2} GB", s as f64 / GB as f64),
        s if s >= MB => format!("{:.2} MB", s as f64 / MB as f64),
        s if s >= KB => format!("{:.2} KB", s as f64 / KB as f64),
        _ => format!("{} B", size),
    }
}

pub fn sort_files_to_folders(input_dir: &Path, output_dir: &Path, failed_guess_paths: &Vec<PathBuf>, separate_wa_sc: bool) -> SortCounts {
    let media_extensions = vec![
        // Images
        "jpg", "jpeg", "png", "webp", "heic", "heif", "bmp", "tiff", "gif", "avif", "jxl", "jfif",
        // Videos
        "mp4", "mov", "mkv", "avi", "webm", "3gp", "m4v", "mpg", "mpeg", "mts", "m2ts", "ts", "flv",
        "f4v", "wmv", "asf", "rm", "rmvb", "vob", "ogv", "mxf", "dv", "divx", "xvid"
    ];

    let logs_dir = output_dir.join("Technical Files").join("logs");
    
    // Set up the two destination folders
    let media_folder = output_dir.join("Media Files");
    let unknown_folder = output_dir.join("Unknown Time");
    let _ = fs::create_dir_all(&media_folder);
    let _ = fs::create_dir_all(&unknown_folder);

    let all_files: Vec<_> = walkdir::WalkDir::new(input_dir).into_iter().filter_map(Result::ok).filter(|e| e.path().is_file()).collect();
    let all_media_files: Vec<_> = all_files.into_iter().filter(|entry| {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            media_extensions.contains(&ext.as_str())
        } else {
            false
        }
    }).collect();

    let total = all_media_files.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.yellow/white}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("🟨⬜")
    );
    pb.set_message("Sorting files...");

    // Now tracks PathBuf instead of Strings to support uniqueness across split directories
    let claimed_paths = Mutex::new(HashSet::new());
    let live_photo_map = Mutex::new(HashMap::new()); // Tracks iOS Live Photo pairs
    let log_mutex = Mutex::new(());

    // Process files in parallel, fold data locally for each thread, reduce everything back together at the end
    let final_state = all_media_files.into_par_iter().fold(
        || SortState::new(),
        |mut local_state, entry| {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            
            let output = get_exiftool_command()
                .arg("-DateTimeOriginal")
                .arg("-MIMEType")
                .arg("-ImageSize")
                .arg("-FileType")
                .arg(path)
                .output();

            let mut date_str = String::new();
            let mut mime_type = String::new();
            let mut image_size = String::new();
            let mut file_type = String::new();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    if line.contains("Date/Time Original") {
                        date_str = line.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string();
                    } else if line.contains("MIME Type") {
                        mime_type = line.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string();
                    } else if line.contains("Image Size") {
                        image_size = line.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string();
                    } else if line.contains("File Type") {
                        file_type = line.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string();
                    }
                }
            }

            if date_str.is_empty() {
                let filename_str = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let parent = path.parent().unwrap_or_else(|| Path::new(""));
                let mut found_json = None;

                let exact = path.with_file_name(format!("{}.json", filename_str));
                let alt = path.with_extension("json");

                if exact.exists() {
                    found_json = Some(exact);
                } else if alt.exists() {
                    found_json = Some(alt);
                } else {
                    let prefix = format!("{}.", filename_str);
                    if let Ok(entries) = fs::read_dir(parent) {
                        for e in entries.flatten() {
                            if let Some(name) = e.file_name().to_str() {
                                if name.starts_with(&prefix) && name.ends_with(".json") {
                                    found_json = Some(e.path());
                                    break;
                                }
                            }
                        }
                    }
                }

                if let Some(json_path) = found_json {
                    if let Ok(json_str) = fs::read_to_string(&json_path) {
                        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                            if let Some(ts) = json_val["photoTakenTime"]["timestamp"].as_str() {
                                if let Ok(timestamp) = ts.parse::<i64>() {
                                    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
                                    date_str = dt.format("%Y:%m:%d %H:%M:%S").to_string();
                                }
                            }
                        }
                    }
                }
            }

            // If an .MP4 component of a Live Photo is missing a date, borrow it from the .HEIC sibling
            if date_str.is_empty() && ext == "mp4" {
                let heic_path = path.with_extension("heic");
                let heic_path_upper = path.with_extension("HEIC");
                
                let sibling = if heic_path.exists() {
                    Some(heic_path)
                } else if heic_path_upper.exists() {
                    Some(heic_path_upper)
                } else {
                    None
                };

                if let Some(hp) = sibling {
                    // 1. Check EXIF of the HEIC sibling
                    if let Ok(heic_out) = get_exiftool_command().arg("-DateTimeOriginal").arg(&hp).output() {
                        let stdout = String::from_utf8_lossy(&heic_out.stdout);
                        for line in stdout.lines() {
                            if line.contains("Date/Time Original") {
                                date_str = line.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string();
                            }
                        }
                    }

                    // 2. Check JSON of the HEIC sibling if EXIF was empty
                    if date_str.is_empty() {
                        let h_filename = hp.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        let h_parent = hp.parent().unwrap_or_else(|| Path::new(""));
                        let mut h_json = None;

                        let exact = hp.with_file_name(format!("{}.json", h_filename));
                        let alt = hp.with_extension("json");

                        if exact.exists() {
                            h_json = Some(exact);
                        } else if alt.exists() {
                            h_json = Some(alt);
                        } else {
                            let prefix = format!("{}.", h_filename);
                            if let Ok(entries) = fs::read_dir(h_parent) {
                                for e in entries.flatten() {
                                    if let Some(name) = e.file_name().to_str() {
                                        if name.starts_with(&prefix) && name.ends_with(".json") {
                                            h_json = Some(e.path());
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(json_path) = h_json {
                            if let Ok(json_str) = fs::read_to_string(&json_path) {
                                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                                    if let Some(ts) = json_val["photoTakenTime"]["timestamp"].as_str() {
                                        if let Ok(timestamp) = ts.parse::<i64>() {
                                            let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
                                            date_str = dt.format("%Y:%m:%d %H:%M:%S").to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let file_size = path.metadata().map(|m| m.len()).unwrap_or(0);
            let raw_filename = path.file_name().unwrap().to_string_lossy().to_string();
            
            // Route to correct folder depending on whether a timestamp was found
            let target_folder = if date_str.is_empty() {
                &unknown_folder
            } else {
                &media_folder
            };

            // Atomically generate and claim unique filename within the specific target directory
            let dest_path = get_unique_dest_path(path, target_folder, &claimed_paths, &live_photo_map);
            let unique_filename = dest_path.file_name().unwrap().to_string_lossy().to_string();

            let fname_lc = raw_filename.to_lowercase();
            let is_wa = fname_lc.contains("wa") || fname_lc.contains("whatsapp");
            let is_sc = fname_lc.contains("screenshot");
            
            let file_info = (unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size);

            if separate_wa_sc && is_wa {
                local_state.counts.whatsapp += 1;
                local_state.photos_info.push(file_info);
            } else if separate_wa_sc && is_sc {
                local_state.counts.screenshots += 1;
                local_state.photos_info.push(file_info);
            } else if ext == "mkv" {
                local_state.counts.mkv += 1;
                local_state.mkv_info.push(file_info);
            } else if date_str.is_empty() {
                local_state.counts.unknown += 1;
                if failed_guess_paths.contains(&path.to_path_buf()) {
                    local_state.failed_guess_info.push(file_info);
                } else {
                    local_state.unknown_info.push(file_info);
                }
            } else if mime_type.starts_with("video") || ["mp4","mov","avi","webm","3gp","m4v","mpg","mpeg","mts","m2ts","ts","flv","f4v","wmv","asf","rm","rmvb","vob","ogv","mxf","dv","divx","xvid"].contains(&ext.as_str()) {
                local_state.counts.videos += 1;
                local_state.videos_info.push(file_info);
            } else {
                local_state.counts.photos += 1;
                local_state.photos_info.push(file_info);
            }

            match fs::copy(path, &dest_path) {
                Ok(_) => {
                    let _lock = log_mutex.lock().unwrap();
                    log_to_file(&logs_dir, "sorting.log", &format!("Copied {:?} to {:?}", raw_filename, dest_path));
                }
                Err(e) => {
                    let _lock = log_mutex.lock().unwrap();
                    log_to_file(&logs_dir, "sorting.log", &format!("Failed to copy {:?} to {:?}: {}", raw_filename, dest_path, e));
                }
            }

            pb.inc(1);
            local_state
        }
    ).reduce(
        || SortState::new(),
        |mut merged, state| {
            merged.merge(state);
            merged
        }
    );

    pb.finish_with_message("Sorting complete!");

    let csv_report_folder = output_dir.join("Technical Files").join("CSV Report");
    let _ = fs::create_dir_all(&csv_report_folder);
    csv_report::write_csv_report(&csv_report_folder, &final_state.photos_info, "photos.csv");
    csv_report::write_csv_report(&csv_report_folder, &final_state.videos_info, "videos.csv");
    csv_report::write_csv_report(&csv_report_folder, &final_state.unknown_info, "unknown_time.csv");
    csv_report::write_csv_report(&csv_report_folder, &final_state.mkv_info, "mkv_files.csv");
    csv_report::write_csv_report(&csv_report_folder, &final_state.failed_guess_info, "failed_filename_guess.csv");
    
    log_to_file(&logs_dir, "sorting.log", "CSV reports written for all categories.");
    
    let total_processed = final_state.counts.photos + final_state.counts.videos + final_state.counts.whatsapp + final_state.counts.screenshots + final_state.counts.unknown + final_state.counts.mkv;
    println!("\n📦 Sorting complete! Copied {} files to Media Files and Unknown Time folders.", total_processed);
    println!("\n📄 CSV files are added in: {}\nPlease keep this folder safe for future use!", csv_report_folder.display());

    final_state.counts
}