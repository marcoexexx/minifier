use std::fs;
use std::io::Write;
use std::path::Path;

const URL: &str = "https://www.toptal.com/developers/javascript-minifier/api/raw";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run --release /path/to/my_js.js");
        std::process::exit(1);
    }

    let js_file_path = &args[1];

    let js_code = fs::read_to_string(js_file_path)?;

    let client = reqwest::Client::new();
    let min = client
        .post(URL)
        .form(&[("input", js_code)])
        .send()
        .await?
        .text()
        .await?;

    let file_name = Path::new(js_file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let min_file_path = format!("{}.min.js", file_name);
    let mut min_file = fs::File::create(&min_file_path)?;

    min_file.write_all(min.as_bytes())?;

    println!("Minified JS saved to: {}", min_file_path);

    Ok(())
}
