use std::{fs, io};
use std::path::Path;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn generate_random_file_name() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}

fn rename_files_in_dir(dir: &Path) -> io::Result<()> {
    let paths = fs::read_dir(dir)?;
    for path in paths {
        let path = path?.path();

        if path.is_file() {
            let new_name = generate_random_file_name();
            let extention = path.extension().and_then(|ext| ext.to_str());
            let new_file_name = match extention {
                Some(ext) => format!("{}.{}", new_name, ext),
                None => new_name,
            };

            let new_path = path.with_file_name(new_file_name);
            fs::rename(&path, &new_path)?;
            println!("Renamed {:?} to {:?}", path, new_path);
        }
    }

    Ok(())
}

pub fn run(dir: &Path) -> io::Result<()> {
    rename_files_in_dir(&dir)
}
