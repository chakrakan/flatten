use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: flatten <dir>");
        return Ok(());
    }

    let root_path = Path::new(&args[1]);
    let root_path_str = root_path.to_str().unwrap();

    let paths = fs::read_dir(&root_path)?;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();

    for path in paths {
        let path = path?;
        if path.path().is_dir() {
            dirs.push(path.path().clone());
            traverse(&path.path(), &mut files)?;
        } else {
            files.push(path.path());
        }
    }

    for file in files {
        let file_name = file.file_name().unwrap();
        let dest_path = Path::new(root_path_str).join(file_name);
        fs::rename(file, &dest_path)?;
    }

    // reverse sort directories by their depth, deepest first
    dirs.sort_by(|a, b| b.components().count().cmp(&a.components().count()));
    for dir in dirs {
        fs::remove_dir_all(dir)?;
    }

    Ok(())
}

fn traverse(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                traverse(&path, files)?;
            } else {
                files.push(path);
            }
        }
    }
    Ok(())
}
