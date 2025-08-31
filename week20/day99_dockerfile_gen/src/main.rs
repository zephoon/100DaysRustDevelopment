use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let cargo_path = Path::new("Cargo.toml");
    if !cargo_path.exists() {
        eprintln!("❌ Not a Rust project (Cargo.toml not found).");
        return;
    }
    let app_name = infer_crate_name().unwrap_or("rust_app".to_string());
    let dockerfile = format!(
        r#"FROM rust:1.74 AS builder
WORKDIR /usr/src/{0}
COPY . .
RUN cargo build --release
 
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/{0}/target/release/{0} /app/{0}
CMD ["./{0}"]
"#,
        app_name
    );
    fs::write("Dockerfile", dockerfile).expect("❌ Failed to write Dockerfile");
    println!("✅ Dockerfile generated for `{}`!", app_name);
}

fn infer_crate_name() -> Option<String> {
    let contents = fs::read_to_string("Cargo.toml").ok()?;
    for line in contents.lines() {
        if line.trim_start().starts_with("name") {
            return line
                .split('=')
                .nth(1)
                .map(|s| s.trim_matches(&[' ', '"'][..]).to_string());
        }
    }
    None
}
