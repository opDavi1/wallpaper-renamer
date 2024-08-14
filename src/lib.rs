use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::collections::HashSet;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};

fn generate_random_file_name(existing_names: &HashSet<String>) -> String {
    loop {
        let random_name = thread_rng() 
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        
        if !existing_names.contains(&random_name) {
            return random_name;
        }
    }
}

fn hash_file_content(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = format!("{:x}", hasher.finalize());
    Ok(hash)
}

fn rename_files_in_dir(dir: &Path) -> io::Result<()> {
    let paths = fs::read_dir(dir)?;
    let mut existing_names = HashSet::new();

    for path in paths {
        let path = path?.path();

        if path.is_file() {
            let new_name = generate_random_file_name(&existing_names); 
            let extention = path.extension().and_then(|ext| ext.to_str());
            let new_file_name = match extention {
                Some(ext) => format!("{}.{}", new_name, ext),
                None => new_name.clone(),
            };

            let new_path = path.with_file_name(new_file_name);
            fs::rename(&path, &new_path)?;
            existing_names.insert(new_name);
            println!("Renamed {:?} to {:?}", path, new_path);
        }
    }

    Ok(())
}

fn delete_duplicate_files(dir: &Path)  -> io::Result<()> {
    let paths = fs::read_dir(dir)?;
    let mut file_hash_map = HashMap::new();

    for path in paths {
        let path = path?.path();
        let file_hash = hash_file_content(&path)?;
        if let Some(_) = file_hash_map.get(&file_hash) {
            fs::remove_file(&path)?;
            println!("Duplicate wallpaper {:?} found. Deleted.", path.file_name().unwrap());
        } else {
            file_hash_map.insert(file_hash, path);
        }
    }   

    Ok(())
}

pub fn run(dir: &Path) -> io::Result<()> {
    rename_files_in_dir(&dir)?;
    delete_duplicate_files(&dir)
}
