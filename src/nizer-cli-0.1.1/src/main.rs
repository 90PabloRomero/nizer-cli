use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use chrono::{DateTime, Local};
use std::io;
use std::collections::HashMap;

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

    // Load last viewed times from .nizer_vxuv
    let mut vxuv_map = load_vxuv(path.parent().unwrap_or(Path::new(".")));
    let vxuv = vxuv_map.remove(path.to_str().unwrap_or("")).unwrap_or_else(|| "N/A".to_string());

    let m = modified.unwrap_or("????-??-?? ??:??:??".into());
    let c = created.unwrap_or("????-??-?? ??:??:??".into());

    format!("{:?} - VXUV: {} - M: {} - C: {}", path.file_name().unwrap(), vxuv, m, c)
}

// Load last viewed times from .nizer_vxuv in the directory
fn load_vxuv(dir: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let vxuv_path = dir.join(".nizer_vxuv");
    if let Ok(content) = fs::read_to_string(&vxuv_path) {
        for line in content.lines() {
            if let Some((file, ts)) = line.split_once('|') {
                map.insert(file.to_string(), ts.to_string());
            }
        }
    }
    map
}

// Save last viewed time for a file
fn save_vxuv(path: &Path) {
    let dir = path.parent().unwrap_or(Path::new("."));
    let vxuv_path = dir.join(".nizer_vxuv");
    let mut map = load_vxuv(dir);
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    map.insert(path.to_str().unwrap_or("").to_string(), now);

    let content = map.iter().map(|(k, v)| format!("{}|{}", k, v)).collect::<Vec<_>>().join("\n");
    let _ = fs::write(vxuv_path, content);
}

fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn open_in_micro(path: &Path) {
    let _ = Command::new("micro")
        .arg(path)
        .status();

    // Update last viewed time
    save_vxuv(path);

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
                    // Prompt for special commands
                    println!("Seleccionaste: {}\nOpciones: [Enter] abrir, --r renombrar, --d borrar, cualquier otra tecla para cancelar", selected.display());
                    let mut cmd = String::new();
                    io::stdin().read_line(&mut cmd)?;
                    let cmd = cmd.trim();
                    if cmd == "--r" {
                        println!("Nuevo nombre para {}:", selected.display());
                        let mut new_name = String::new();
                        io::stdin().read_line(&mut new_name)?;
                        let new_name = new_name.trim();
                        if !new_name.is_empty() {
                            let new_path = selected.parent().unwrap().join(new_name);
                            if let Err(e) = fs::rename(selected, &new_path) {
                                println!("Error al renombrar: {}", e);
                            } else {
                                println!("Renombrado exitoso.");
                            }
                        }
                    } else if cmd == "--d" {
                        println!("¿Seguro que quieres borrar {}? (s/N)", selected.display());
                        let mut confirm = String::new();
                        io::stdin().read_line(&mut confirm)?;
                        if confirm.trim().to_lowercase() == "s" {
                            if let Err(e) = fs::remove_file(selected) {
                                println!("Error al borrar: {}", e);
                            } else {
                                println!("Archivo borrado.");
                            }
                        }
                    } else if cmd == "" {
                        open_in_micro(selected);
                    }
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

