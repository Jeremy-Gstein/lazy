pub fn lazy_dockerfile(image: &str) -> String {
    format!(r#"FROM {} AS build
RUN apk add --no-cache \
  python3 \
  py3-pip 

WORKDIR /app

CMD ["echo", "Hello World"]
"#, image)
}

pub fn package_manager(image: &str) -> String {
    // match os and return its native package manager
    let mut lazy_package_manager = String::new();
    match image {
        "alpine" => lazy_package_manager.push_str("apk"),
        "arch" => lazy_package_manager.push_str("pacman"),
        // we assume debian based (apt) as default.
        _ => lazy_package_manager.push_str("apt"),
    }
    lazy_package_manager
}
