use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fs;
use chrono::{TimeZone, Utc};
use crate::csv_report;
use crate::utils::log_to_file;
use serde_json;
use crate::platform::get_exiftool_command;

pub struct SortCounts {
    pub photos: usize,
    pub videos: usize,
    pub whatsapp: usize,
    pub screenshots: usize,
    pub unknown: usize,
    pub mkv: usize,
}

/// Generates a unique filename inside dest_dir by appending _1, _2, etc. if duplicates exist.
fn get_unique_filename(dest_dir: &Path, original_filename: &str) -> String {
    let candidate = dest_dir.join(original_filename);
    if !candidate.exists() {
        return original_filename.to_string();
    }

    let path_ref = Path::new(original_filename);
    let stem = path_ref.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = path_ref.extension().and_then(|e| e.to_str());

    let mut counter = 1;
    loop {
        let new_name = match ext {
            Some(e) => format!("{}_{}.{}", stem, counter, e),
            None => format!("{}_{}", stem, counter),
        };
        if !dest_dir.join(&new_name).exists() {
            return new_name;
        }
        counter += 1;
    }
}

/// Main function to organize files into the flat Media Files folder.
pub fn sort_files_to_folders(input_dir: &Path, output_dir: &Path, failed_guess_paths: &Vec<PathBuf>, separate_wa_sc: bool) -> SortCounts {
    let media_extensions = vec![
        // Images
        "jpg", "jpeg", "png", "webp", "heic", "heif", "bmp", "tiff", "gif", "avif", "jxl", "jfif",
        // Videos
        "mp4", "mov", "mkv", "avi", "webm", "3gp", "m4v", "mpg", "mpeg", "mts", "m2ts", "ts", "flv",
        "f4v", "wmv", "asf", "rm", "rmvb", "vob", "ogv", "mxf", "dv", "divx", "xvid"
    ];

    let mut photos_info = Vec::new();
    let mut videos_info = Vec::new();
    let mut unknown_info = Vec::new();
    let mut mkv_info = Vec::new();
    let mut failed_guess_info = Vec::new();

    let mut counts = SortCounts {
        photos: 0,
        videos: 0,
        whatsapp: 0,
        screenshots: 0,
        unknown: 0,
        mkv: 0,
    };

    let logs_dir = output_dir.join("Technical Files").join("logs");
    let dest_folder = output_dir.join("Media Files");
    let _ = fs::create_dir_all(&dest_folder);

    let all_files: Vec<_> = walkdir::WalkDir::new(input_dir).into_iter().filter_map(Result::ok).filter(|e| e.path().is_file()).collect();
    let all_media_files: Vec<_> = all_files.iter().filter(|entry| {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            media_extensions.contains(&ext.as_str())
        } else {
            false
        }
    }).collect();

    let total = all_media_files.len();
    let mut processed = 0;

    for entry in all_media_files {
        let path = entry.path();
        if path.is_file() {
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

            // If date_str is still empty, try to extract from matching JSON
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

            let file_size = path.metadata().map(|m| m.len()).unwrap_or(0);
            let raw_filename = path.file_name().unwrap().to_string_lossy().to_string();
            let unique_filename = get_unique_filename(&dest_folder, &raw_filename);
            let dest_path = dest_folder.join(&unique_filename);

            let fname_lc = raw_filename.to_lowercase();
            let is_wa = fname_lc.contains("wa") || fname_lc.contains("whatsapp");
            let is_sc = fname_lc.contains("screenshot");

            if separate_wa_sc && is_wa {
                counts.whatsapp += 1;
                photos_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
            } else if separate_wa_sc && is_sc {
                counts.screenshots += 1;
                photos_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
            } else if ext == "mkv" {
                counts.mkv += 1;
                mkv_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
            } else if date_str.is_empty() {
                counts.unknown += 1;
                if failed_guess_paths.contains(&path.to_path_buf()) {
                    failed_guess_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
                } else {
                    unknown_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
                }
            } else if mime_type.starts_with("video") || ["mp4","mov","avi","webm","3gp","m4v","mpg","mpeg","mts","m2ts","ts","flv","f4v","wmv","asf","rm","rmvb","vob","ogv","mxf","dv","divx","xvid"].contains(&ext.as_str()) {
                counts.videos += 1;
                videos_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
            } else {
                counts.photos += 1;
                photos_info.push((unique_filename.clone(), file_type.clone(), date_str.clone(), image_size.clone(), human_readable_size(file_size), file_size));
            }

            match fs::copy(path, &dest_path) {
                Ok(_) => {
                    log_to_file(&logs_dir, "sorting.log", &format!("Copied {:?} to {:?}", raw_filename, dest_path));
                }
                Err(e) => {
                    log_to_file(&logs_dir, "sorting.log", &format!("Failed to copy {:?} to {:?}: {}", raw_filename, dest_path, e));
                }
            }

            processed += 1;
            print_progress(processed, total);
        }
    }

    let csv_report_folder = output_dir.join("Technical Files").join("CSV Report");
    let _ = fs::create_dir_all(&csv_report_folder);
    csv_report::write_csv_report(&csv_report_folder, &photos_info, "photos.csv");
    csv_report::write_csv_report(&csv_report_folder, &videos_info, "videos.csv");
    csv_report::write_csv_report(&csv_report_folder, &unknown_info, "unknown_time.csv");
    csv_report::write_csv_report(&csv_report_folder, &mkv_info, "mkv_files.csv");
    csv_report::write_csv_report(&csv_report_folder, &failed_guess_info, "failed_filename_guess.csv");
    
    log_to_file(&logs_dir, "sorting.log", "CSV reports written for all categories.");
    println!("\n📦 Sorting complete! Copied {} files to flat Media Files folder.", processed);
    println!("\n📄 CSV files are added in: {}\nPlease keep this folder safe for future use!", csv_report_folder.display());

    counts
}

fn print_progress(done: usize, total: usize) {
    let percent = if total > 0 { (done * 100) / total } else { 100 };
    let bar = format!("{}{}", "🟨".repeat(percent / 4), "⬜".repeat(25 - percent / 4));
    print!("\r📦 Sorting: [{}] {}% ({} / {})", bar, percent, done, total);
    let _ = io::stdout().flush();
    if done == total {
        println!();
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