use std::process::Command;
use crate::LazyConfig;
use std::path::Path;

pub fn run(config: &LazyConfig) {
    // Optionally check if the Docker image exists (simplified check)
    let image_exists = {
        let output = Command::new("docker")
            .arg("images")
            .arg("-q")           // quiet IDs
            .arg(&config.image)
            .output()
            .expect("Failed to check docker images");
        !output.stdout.is_empty()
    };

    if !image_exists {
        eprintln!("Warning: Docker image '{}' not found. Please run build first.", config.image);
        return;
    }

    // Compose docker run command
    let mut cmd = Command::new("docker");
    cmd.arg("run")
        .arg("-itd")                    // interactive, tty, detached (customize as needed)
        .arg("--name")
        .arg(&config.name);

    // Add --env-file if specified and exists
    if !config.env.is_empty() && Path::new(&config.env).exists() {
        cmd.arg("--env-file").arg(&config.env);
        println!("Running container with env file: {}", &config.env);
    }

    cmd.arg(&config.image);

    // Run the command and check status
    let status = cmd.status().expect("Failed to execute docker run");

    if status.success() {
        println!("Container '{}' started successfully from image '{}'.", 
            &config.name, &config.image);

        // Optionally show running containers
        let _ = Command::new("docker").arg("ps").status();
    } else {
        eprintln!("Failed to start container '{}'.", &config.name);
    }
}
