use std::process::Command;
use std::path::Path;
use crate::LazyConfig;
use crate::utils::lazy_dockerfile;

pub fn run(config: &LazyConfig) {
    if Path::new(&config.dockerfile).exists() {
        println!("Building from Dockerfile at {}", config.dockerfile);
        let status = Command::new("docker")
            .arg("build")
            .arg("-f")
            .arg(&config.dockerfile)
            .arg("-t")
            .arg(&config.image)
            .arg(".")
            .status()
            .expect("Failed to run docker build");
        if !status.success() {
            eprintln!("docker build failed");
        }
    } else {
        println!("Generating default Dockerfile and building...");
        let dockerfile_contents = lazy_dockerfile(&config.image);
        let mut child = Command::new("docker")
            .arg("build")
            .arg("-f")
            .arg("-")
            .arg("-t")
            .arg(&config.image)
            .arg(".")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn docker build");
        use std::io::Write;
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(dockerfile_contents.as_bytes()).unwrap();
        drop(stdin);
        let output = child.wait().expect("Failed to wait on docker build");
        if !output.success() {
            eprintln!("docker build failed");
        }
    }
}



// pub fn run() {
//     println!("Build logic.");
//     // Actual remove logic (e.g., Command::new("docker").arg("build")...)
// }
