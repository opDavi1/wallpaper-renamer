// This file is part of wprn by opDavi1, licensed under the MIT License.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{fs, io};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};


pub struct Config {
    dir: String,
    file_name_length: usize,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Error: Not enough arguments!")
        } else if args.len() == 2 {
            let dir = args[1].clone();
            return Ok(Config {dir, file_name_length: 10})
        } else {
            let dir = args[1].clone();
            let file_name_length: usize = match args[2].clone().parse() {
                Ok(i) => i,
                Err(_) => return Err("Error: File Name Length not an integer!"),
            };
            return Ok(Config {dir, file_name_length});
        }
    }
}


fn generate_random_file_name(length: &usize, existing_names: &HashSet<String>) -> String {
    loop {
        let random_name = thread_rng() 
            .sample_iter(&Alphanumeric)
            .take(*length)
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


fn rename_files_in_dir(dir: &String, file_name_length: &usize) -> io::Result<()> {
    let paths = fs::read_dir(dir)?;
    let mut existing_names = HashSet::new();

    for path in paths {
        let path = path?.path();
        let current_file_name = match path.file_stem() {
            Some(n) => n,
            None => {
                return io::Result::Err(io::Error::new(io::ErrorKind::InvalidData, "Could not read file name"));
            },
        };

        if path.is_file() && current_file_name.len() != *file_name_length {
            let new_name = generate_random_file_name(file_name_length, &existing_names); 
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


fn delete_duplicate_files(dir: &String)  -> io::Result<()> {
    let paths = fs::read_dir(dir)?;
    let mut file_hash_map = HashMap::new();

    for path in paths {
        let path = path?.path();
        if path.is_file() {
            let file_name = match path.file_name() {
                Some(n) => n,
                None => return io::Result::Err(io::Error::new(io::ErrorKind::InvalidData, "Error: Could not read file name!")),
            };
            let file_hash = hash_file_content(&path)?;
            if let Some(_) = file_hash_map.get(&file_hash) {
                fs::remove_file(&path)?;
                println!("Duplicate wallpaper {:?} found. Deleted.", file_name);
            } else {
                file_hash_map.insert(file_hash, path);
            }
        }
    }   

    Ok(())
}


pub fn run(config: Config) -> io::Result<()> {
    delete_duplicate_files(&config.dir)?;
    rename_files_in_dir(&config.dir, &config.file_name_length)
}


pub fn show_usage() {
    println!("---------------------------------------------------------
    Usage: wprn <Directory> [File Name Length]
---------------------------------------------------------
Wallpaper Renamer (wprn) by opDavi1
Renames all files in a given folder to a random string and deletes any duplicate files found.
    
This program is distributed under the MIT license. See the included LICENSE.md file for more information or use the --license flag while running this command.");
}


pub fn show_license() {
    println!("This program is licensed under the MIT license. See the included LICENSE.md file for a copy of this license.
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Copyright © 2024 Davis Sherman (opDavi1) 

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
}
