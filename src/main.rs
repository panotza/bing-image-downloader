use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Image directory required".into());
    }

    if args.len() < 3 {
        return Err("Image resolution required".into());
    }

    let image_dir = PathBuf::from(args[1].clone());
    if !image_dir.is_dir() {
        return Err("Path is not image directory".into());
    }

    let req_resolution = args[2].clone();

    let body = reqwest::get("https://www.bing.com").await?.text().await?;
    let re_html_tag = Regex::new(r#"<link[^>]+id="preloadBg"[^>]+/>"#)?;
    let html_tag = re_html_tag.captures(body.as_str()).unwrap().get(0).unwrap().as_str();
    let re_href = Regex::new(r#"href="([^"]+)""#)?;
    let href = re_href.captures(html_tag).unwrap().get(1).unwrap().as_str();
    let re_file_name = Regex::new(r"\w+.jpg")?;
    let re_size = Regex::new(r"(\d+x\d+).jpg")?;
    let original_resolution = re_size.captures(href).unwrap().get(1).unwrap().as_str();
    let file_name = re_file_name.captures(href).unwrap().get(0).unwrap().as_str().replace(original_resolution, req_resolution.as_str());

    let image_url = format!("https://www.bing.com{}", href.replace(original_resolution, req_resolution.as_str()));
    let resp = reqwest::get(image_url.as_str()).await?;
    if resp.status() != 200 {
        return Err("Image not found".into());
    }
    let image_bytes = resp.bytes().await?;
    let image_path = image_dir.join(file_name);
    if image_path.exists() {
        return Ok(());
    }

    let mut f = File::create(image_path)?;
    f.write_all(image_bytes.as_ref())?;
    Ok(())
}
