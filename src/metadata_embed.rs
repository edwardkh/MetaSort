// metadata_embed.rs
// Embedding metadata logic for MetaSort_v1.0.0 – Google Photos Takeout Organizer 

use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::sync::Mutex;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::metadata_extraction::MediaMetadata;
use crate::filename_date_guess::extract_date_from_filename;
use crate::utils::log_to_file;
use crate::platform::get_exiftool_command;

pub fn embed_metadata_all(metadata_list: &[MediaMetadata], log_dir: &Path) {
    let logs_dir = log_dir.join("logs");
    let log_path = logs_dir.join("metadata_embedding.log");
    let _ = fs::create_dir_all(&logs_dir);
    let _log_file = File::create(&log_path).expect("Failed to create log file");
    
    println!("\n🧐 Do you want to embed date/time for WhatsApp & Screenshot images based on their:\n1. Metadata\n2. Filename\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let use_filename = matches!(input.trim(), "2");
    
    let total = metadata_list.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.blue/white}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("🟦⬜")
    );
    pb.set_message("Embedding metadata...");

    // Lock for logging to prevent concurrent write collisions
    let log_mutex = Mutex::new(());

    metadata_list.par_iter().for_each(|meta| {
        let mut args = Vec::new();
        let filename = meta.media_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let parent = meta.media_path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("");
        let is_wa = parent.eq_ignore_ascii_case("Whatsapp");
        let is_sc = parent.eq_ignore_ascii_case("Screenshots");
        
        let mut used = "metadata";
        let mut date_to_embed = meta.exif_date.clone();
        
        if use_filename && (is_wa || is_sc) {
            if let Some(date) = extract_date_from_filename(filename) {
                date_to_embed = Some(date);
                used = "filename";
            }
        }
        if date_to_embed.is_none() {
            used = "metadata (fallback)";
        }
        
        if let Some(ref date) = date_to_embed {
            args.push(format!("-EXIF:DateTimeOriginal={}", date));
        }
        if let (Some(lat), Some(lon)) = (meta.gps_latitude, meta.gps_longitude) {
            args.push(format!("-GPSLatitude={}", lat));
            args.push(format!("-GPSLongitude={}", lon));
        }
        if let Some(alt) = meta.gps_altitude {
            args.push(format!("-GPSAltitude={}", alt));
        }
        if let Some(ref make) = meta.camera_make {
            args.push(format!("-Make={}", make));
        }
        if let Some(ref model) = meta.camera_model {
            args.push(format!("-Model={}", model));
        }
        
        args.push("-overwrite_original".to_string());
        args.push(meta.media_path.to_string_lossy().to_string());
        
        let log_msg = format!(
            "File: {:?}, Used: {}, Date: {:?}, Lat: {:?}, Lon: {:?}, Alt: {:?}, Make: {:?}, Model: {:?}",
            meta.media_path.file_name().unwrap_or_default(), used, date_to_embed, meta.gps_latitude, meta.gps_longitude, meta.gps_altitude, meta.camera_make, meta.camera_model
        );
        
        let mut retry_needed = false;
        let mut new_media_path = meta.media_path.clone();

        match get_exiftool_command().args(&args).output() {
            Ok(output) => {
                let _lock = log_mutex.lock().unwrap();
                if output.status.success() {
                    log_to_file(&logs_dir, "metadata_embedding.log", &format!("✅ Embedded metadata. {}", log_msg));
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
                    if stderr.contains("looks more like a") {
                        if let Some(idx) = stderr.find("looks more like a ") {
                            let start = idx + "looks more like a ".len();
                            let ext_end = stderr[start..].find(|c: char| !c.is_alphabetic()).unwrap_or(stderr.len() - start);
                            let correct_ext = &stderr[start..start + ext_end];
                            
                            new_media_path = meta.media_path.with_extension(correct_ext);
                            
                            if fs::rename(&meta.media_path, &new_media_path).is_ok() {
                                log_to_file(&logs_dir, "metadata_embedding.log", &format!("⚠️  Renamed {:?} to {:?} due to extension mismatch.", meta.media_path.file_name().unwrap_or_default(), new_media_path.file_name().unwrap_or_default()));
                                retry_needed = true;
                            } else {
                                log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Failed to embed metadata. Could not rename mismatched file. {}", log_msg));
                            }
                        }
                    } else {
                        log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Failed to embed metadata. Stderr: {}. {}", stderr.trim(), log_msg));
                    }
                }
            }
            Err(e) => {
                let _lock = log_mutex.lock().unwrap();
                log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Error running exiftool. Error: {}. {}", e, log_msg));
            }
        }

        if retry_needed {
            args.pop();
            args.push(new_media_path.to_string_lossy().to_string());
            
            match get_exiftool_command().args(&args).output() {
                Ok(out) => {
                    let _lock = log_mutex.lock().unwrap();
                    if out.status.success() {
                        log_to_file(&logs_dir, "metadata_embedding.log", &format!("✅ Embedded metadata on retry after rename. File: {:?}", new_media_path.file_name().unwrap_or_default()));
                    } else {
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Failed to embed metadata on retry. Stderr: {}. File: {:?}", stderr.trim(), new_media_path.file_name().unwrap_or_default()));
                    }
                },
                Err(e) => {
                    let _lock = log_mutex.lock().unwrap();
                    log_to_file(&logs_dir, "metadata_embedding.log", &format!("❌ Error running exiftool on retry. Error: {}. File: {:?}", e, new_media_path.file_name().unwrap_or_default()));
                }
            }
        }
        
        pb.inc(1);
    });

    pb.finish_with_message("Metadata embedding complete!");
    println!("\n✅ Metadata embedding complete! Embedded metadata for {} files. Log: {:?}", total, log_path);
}