#[macro_use]
extern crate log;

use std::fs::{create_dir_all, File};
use std::{fs, io};
use std::env::current_exe;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use clap::{App, Arg};


pub const MAIN_SEPARATOR: char = std::path::MAIN_SEPARATOR;

fn main() {
    #[cfg(debug_assertions)]
    let log_level = log::LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let log_level = log::LevelFilter::Info;
    
    // Setup logging
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("gtk_icon_debloat", log_level)
        .chain(std::io::stdout())
        .apply()
        .expect("Cannot setup logger");
    
    // Arguments
    let matches = App::new("gtk-icon-debloat")
        .version("1.0.0")
        .author("Teemu N.")
        .about("Copies specified GTK icons to a new directory.")
        .arg(Arg::new("source")
            .short('s')
            .long("source")
            .value_name("PATH")
            .about("Path to GTK icons directory.")
            .required(true)
            .takes_value(true))
        .arg(Arg::new("icons")
            .short('i')
            .long("icons")
            .value_name("FILE")
            .about("Path to a file containing GTK icon names. Looks for `iconlist.txt` by default.")
            .takes_value(true))
        .arg(Arg::new("out")
            .short('o')
            .long("out")
            .value_name("PATH")
            .about("Directory path to which the icon files are copied to. Missing directories will be created. Uses `out` by default.")
            .takes_value(true))
        .get_matches();
    
    if let Some(ref source) = matches.value_of("source") {
        info!("Starting..");
        
        // Path which has all the GTK icons
        let source_path = Path::new(source);
        
        // Icon list file path
        let icons_path = if let Some(icons) = matches.value_of("icons") {
            // Use the supplied iconlist file path
            PathBuf::from(icons)
        } else {
            // Use the default iconlist file path
            match current_exe() {
                Ok(mut path) => {
                    // Remove the actual binary file from the path
                    path.pop();
                    // Add the output directory to the path
                    path.join("iconlist.txt")
                },
                Err(e) => {
                    error!("Could not get the iconlist path.");
                    error!("{:?}", e);
                    return;
                }
            }
        };
        
        // Output directory path
        let out_path = if let Some(out) = matches.value_of("out") {
            // Use the supplied output path
            PathBuf::from(out)
        } else {
            // Default output directory path is the
            // same as this binary file
            match current_exe() {
                Ok(mut path) => {
                    // Remove the actual binary file from the path
                    path.pop();
                    // Add the output directory to the path
                    path.join("out")
                },
                Err(e) => {
                    error!("Could not get the output directory path.");
                    error!("{:?}", e);
                    return;
                }
            }
        };
        
        // Create output directory if it does not exist yet
        if !out_path.exists() {
            match create_dir_all(out_path.clone()) {
                Ok(_) => {},
                Err(e) => {
                    error!("{:?}", e);
                    return;
                }
            }
        }
        
        if icons_path.exists() {
            // Read the file which contains a list of interesting icon names
            let file = File::open(icons_path).expect("No such file.");
            let buf = BufReader::new(file);
            // List of interesting icon names
            let icons_to_get = buf.lines()
               .map(|l| l.expect("Could not parse line"))
               .collect::<Vec<String>>();
            
            // Find all interesting icon paths
            let mut icon_paths: Vec<PathBuf> = vec![];
            match find_icon_paths(source_path, &icons_to_get, &mut icon_paths) {
                Ok(_) => {},
                Err(e) => {
                    error!("{:?}", e);
                    return;
                }
            }
            
            for icon_path in icon_paths.iter_mut() {
                // Full path to the new file which is the copy output
                let new_icon_path = out_path.join(path_subtract(icon_path, source_path));
                // Full path to the new file directory
                let mut new_icon_path_dir = new_icon_path.clone();
                new_icon_path_dir.pop();
                
                // Create any missing directories for the icons
                // that will be copied to the output path
                if !new_icon_path_dir.exists() {
                    match create_dir_all(&new_icon_path_dir) {
                        Ok(_) => {},
                        Err(e) => {
                            error!("Could not create new icon directories: {:?}", new_icon_path_dir);
                            error!("{:?}", e);
                            return;
                        }
                    }
                }
                
                // Copy icons from source to the output
                match fs::copy(&icon_path, &new_icon_path) {
                    Ok(_) => debug!("Copied {:?} to {:?}", icon_path, new_icon_path),
                    Err(e) => error!("{:?}", e)
                }
            }
            
            info!("All done :)");
        } else {
            error!("iconlist.txt could not be found.");
        }
    

    }
    
}

/// Find full paths to the item in `icon_list` and return them.
fn find_icon_paths(dir_path: &Path, icon_list: &[String], icon_paths: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_icon_paths(&path, icon_list, icon_paths)?;
            } else {
                // Found a file so check if the file name is in the list of icons
                for icon_name in icon_list.iter() {
                    if entry.file_name().to_str().unwrap().contains(icon_name) {
                        // Push the full icon path to the list of icon paths
                        icon_paths.push(entry.path());
                    }
                }
            }
        }
    }
    Ok(())
}

/// Subtract `part` from the ´full` path and return a path without leading slashes.
///
/// Example: `/tmp/path/is/a/lie` - `/tmp/path` = `is/a/lie`
fn path_subtract(full: &Path, part: &Path) -> PathBuf {
    let path_items = full
        .to_str()
        .unwrap()
        .replace(part.to_str().unwrap(), "")
        .split(MAIN_SEPARATOR)
        .skip_while(|s| s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    PathBuf::from(path_items.join(&MAIN_SEPARATOR.to_string()))
}