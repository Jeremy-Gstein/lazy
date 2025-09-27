pub fn lazy_dockerfile(image: &str) -> String {
    format!(r#"FROM {} AS build
RUN apk add --no-cache \
  python3 \
  py3-pip 

WORKDIR /app

CMD ["echo", "Hello World"]
"#, image)
}
