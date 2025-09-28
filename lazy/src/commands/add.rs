use crate::config::LazyConfig;
use crate::utils::{package_manager, lazy_dockerfile};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn run(config: &LazyConfig, package: &str) {
    let pkg_manager = package_manager(&config.image_name);
    let dockerfile_path = &config.dockerfile;

    // Ensure parent directories exist
    let path = Path::new(dockerfile_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() && !parent.to_str().unwrap_or("").is_empty() {
            create_dir_all(parent).expect("Failed to create Dockerfile directory");
        }
    }

    // Create default Dockerfile if missing
    if !path.exists() {
        let dockerfile_contents = lazy_dockerfile(&config.image);
        let mut file = File::create(dockerfile_path)
            .expect("Failed to create new Dockerfile.");
        file.write_all(dockerfile_contents.as_bytes())
            .expect("Failed to write default Dockerfile.");
        println!("Created default Dockerfile at {}", dockerfile_path);
    }

    // Read Dockerfile contents into lines
    let file = OpenOptions::new()
        .read(true)
        .open(dockerfile_path)
        .expect("Unable to open Dockerfile");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Failed to read line")).collect();

    let search_prefix = format!("RUN {} add", pkg_manager);
    let new_run_line = format!("RUN {} add --no-cache {}", pkg_manager, package);

    // Find first RUN install line index
    let first_pkg_line_index = lines.iter().position(|l| l.trim_start().starts_with(&search_prefix));

    let mut new_lines = Vec::new();

    match first_pkg_line_index {
        Some(idx) => {
            // Insert new RUN line before the first existing install RUN command
            for (i, line) in lines.iter().enumerate() {
                if i == idx {
                    new_lines.push(new_run_line.clone());
                }
                new_lines.push(line.clone());
            }
        }
        None => {
            // Insert after FROM line or at top if none
            let from_idx = lines.iter().position(|l| l.trim_start().starts_with("FROM"));
            let insert_idx = from_idx.map(|i| i + 1).unwrap_or(0);

            for (i, line) in lines.iter().enumerate() {
                if i == insert_idx {
                    new_lines.push(new_run_line.clone());
                }
                new_lines.push(line.clone());
            }

            // If Dockerfile is empty, create minimal with FROM + RUN
            if lines.is_empty() {
                new_lines.push(format!("FROM {} AS build", config.image));
                new_lines.push(new_run_line);
            }
        }
    }

    // Write updated Dockerfile
    let mut file = File::create(dockerfile_path).expect("Failed to rewrite Dockerfile");
    for line in &new_lines {
        writeln!(file, "{}", line).expect("Failed to write line");
    }

    println!(
        "Inserted '{}' before first {} add RUN line in '{}'",
        package, pkg_manager, dockerfile_path
    );
}

