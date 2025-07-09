use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use chrono::{DateTime, Local};
use std::io;

fn is_text_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(ext.to_str().unwrap_or(""), "md" | "txt" | "doc" | "html")
    } else {
        false
    }
}

fn format_metadata(path: &Path) -> String {
    let metadata = fs::metadata(path).unwrap();
    let modified = metadata.modified().ok().map(format_system_time);
    let created = metadata.created().ok().map(format_system_time);

    let m = modified.unwrap_or("????-??-??".into());
    let c = created.unwrap_or("????-??-??".into());

    format!("{:?} - VXUV: {} - C: {}", path.file_name().unwrap(), m, c)
}

fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d").to_string()
}

fn open_in_micro(path: &Path) {
    let _ = Command::new("micro")
        .arg(path)
        .status();
    
    println!("\x1B[2J\x1B[1;1H"); // Clear screen after exiting micro
}

fn read_and_select(current_dir: &mut PathBuf) -> io::Result<()> {
    loop {
        println!("\n📁 Directorio: {}\n", current_dir.display());

        let mut entries: Vec<PathBuf> = vec![PathBuf::from("..")];
        if let Ok(read_dir) = fs::read_dir(&*current_dir) {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    entries.push(entry.path());
                }
            }
        }

        // Sort entries: directories first, then files
        entries[1..].sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().unwrap_or_default().cmp(&b.file_name().unwrap_or_default())
            }
        });

        // Display entries
        for (i, entry) in entries.iter().enumerate() {
            if entry.as_path().to_str().unwrap() == ".." {
                println!("[{}] 📂 ..", i);
            } else if entry.is_dir() {
                println!("[{}] 📂 {}", i, entry.file_name().unwrap().to_str().unwrap());
            } else if is_text_file(entry) {
                println!("[{}] 📝 {}", i, format_metadata(entry));
            }
        }

        println!("\nSeleccioná un número o `q` para salir:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "q" {
            break;
        }

        if let Ok(index) = input.parse::<usize>() {
            if let Some(selected) = entries.get(index) {
                if selected.as_path().to_str().unwrap() == ".." {
                    if let Some(parent) = current_dir.parent() {
                        *current_dir = parent.to_path_buf();
                    }
                } else if selected.is_dir() {
                    *current_dir = selected.clone();
                } else if is_text_file(selected) {
                    open_in_micro(selected);
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };
    
    // Convert to absolute path
    if !dir.is_absolute() {
        dir = env::current_dir()?.join(dir);
    }
    dir = dir.canonicalize()?;

    read_and_select(&mut dir)
}

