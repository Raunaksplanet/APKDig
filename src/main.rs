use clap::{Arg, Command};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use walkdir::WalkDir;
use rayon::prelude::*;

fn get_ext_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (".env", "env_files"),
        (".conf", "conf_files"),
        (".db", "db_files"),
        (".sqlite", "db_files"),
        (".sqlite3", "db_files"),
        (".bak", "backup_files"),
        (".log", "log_files"),
        (".cfg", "cfg_files"),
        (".yml", "yaml_files"),
        (".yaml", "yaml_files"),
        (".json", "json_files"),
        (".so", "so_files"),
    ])
}

fn print_progress_bar(done: usize, total: usize, length: usize) {
    let percent = done as f64 / total as f64;
    let filled = (length as f64 * percent).round() as usize;
    let bar = "█".repeat(filled) + &"-".repeat(length - filled);
    print!("\r[{}] {:>3}%", bar, (percent * 100.0) as usize);
    io::stdout().flush().unwrap();
}

fn extract_files(decompiled_path: &str) -> io::Result<()> {
    let ext_map = get_ext_map();
    let base_dir = Path::new("extracted_files");
    fs::create_dir_all(base_dir)?;

    let entries: Vec<PathBuf> = WalkDir::new(decompiled_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    let mut file_tasks = Vec::new();
    for path in entries {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let dot_ext = format!(".{}", ext);
            if let Some(folder) = ext_map.get(dot_ext.as_str()) {
                let dest_dir = base_dir.join(folder);
                fs::create_dir_all(&dest_dir)?;
                let filename = path.file_name().unwrap();
                let dest_path = dest_dir.join(filename);
                file_tasks.push((path, dest_path));
            }
        }
    }

    if file_tasks.is_empty() {
        println!("No matching sensitive files found.");
        return Ok(());
    }

    let total = file_tasks.len();
    let progress = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let progress_bar_len = 40;

    file_tasks.par_iter().for_each(|(src, dst)| {
        let _ = fs::copy(src, dst);
        let done = progress.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        print_progress_bar(done, total, progress_bar_len);
    });

    println!("\nTotal files copied: {}", total);
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("apkdig")
        .version("1.0")
        .author("Biscuit Security")
        .about("APK Sensitive File Extractor")
        .arg(
            Arg::new("apk")
                .short('a')
                .long("apk")
                .help("Path to APK file")
                .value_name("APK")
                .num_args(1),
        )
        .arg(
            Arg::new("decompiled")
                .short('d')
                .long("decompiled")
                .help("Path to decompiled APK folder")
                .value_name("FOLDER")
                .num_args(1),
        )
        .get_matches();

    if let Some(apk_path) = matches.get_one::<String>("apk") {
        println!("Decompiling APK: {}", apk_path);
        let out_dir = "out";

        let status = ProcessCommand::new("apktool")
            .args(["d", "-f", apk_path, "-o", out_dir])
            .status()
            .expect("failed to execute apktool");

        if !status.success() {
            eprintln!("apktool failed");
            std::process::exit(1);
        }

        extract_files(out_dir)?;
    } else if let Some(decompiled_path) = matches.get_one::<String>("decompiled") {
        extract_files(decompiled_path)?;
    } else {
        eprintln!("Please provide either -a <apk> or -d <decompiled_folder>");
        std::process::exit(1);
    }

    Ok(())
}
