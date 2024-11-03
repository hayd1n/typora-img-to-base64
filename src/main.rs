use base64::prelude::*;
use image::ImageFormat;
use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

// Determine whether the image has a relative path
fn is_relative_path(src: &str) -> bool {
    !src.starts_with("http://") && !src.starts_with("https://")
}

// Read local image file (including SVG) and convert to Base64
fn read_local_image_as_base64(path: &Path) -> Option<String> {
    if path.extension().and_then(|ext| ext.to_str()) == Some("svg") {
        // For SVG files, read as text
        let mut file = File::open(path).ok()?;
        let mut svg_content = String::new();
        file.read_to_string(&mut svg_content).ok()?;
        let base64_string = BASE64_STANDARD.encode(svg_content.as_bytes());
        return Some(format!("data:image/svg+xml;base64,{}", base64_string));
    }

    // For other image formats, read as binary
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).ok()?;

    // Detect image format
    let format = image::guess_format(&buffer).ok()?;
    let mime_type = match format {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::Bmp => "image/bmp",
        ImageFormat::Tiff => "image/tiff",
        ImageFormat::Ico => "image/x-icon",
        ImageFormat::WebP => "image/webp",
        _ => return None, // Unsupported format
    };

    let base64_string = BASE64_STANDARD.encode(&buffer);
    Some(format!("data:{};base64,{}", mime_type, base64_string))
}

// Download image (including SVG) from the network and convert to Base64
fn download_image_as_base64(url: &str) -> Option<String> {
    let response = get(url).ok()?;
    let bytes = response.bytes().ok()?;

    // Check if SVG by inspecting first bytes
    if url.ends_with(".svg") || (bytes.starts_with(b"<svg") && bytes.ends_with(b"</svg>")) {
        // Convert SVG XML to Base64 as text
        let svg_content = String::from_utf8_lossy(&bytes);
        let base64_string = BASE64_STANDARD.encode(svg_content.as_bytes());
        return Some(format!("data:image/svg+xml;base64,{}", base64_string));
    }

    // Handle binary images
    let bytes_vec = bytes.to_vec();
    let format = image::guess_format(&bytes_vec).ok()?;
    let mime_type = match format {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::Bmp => "image/bmp",
        ImageFormat::Tiff => "image/tiff",
        ImageFormat::Ico => "image/x-icon",
        ImageFormat::WebP => "image/webp",
        _ => return None, // Unsupported format
    };

    let base64_string = BASE64_STANDARD.encode(&bytes_vec);
    Some(format!("data:{};base64,{}", mime_type, base64_string))
}

// Convert all images in <img> tags to Base64
fn convert_img_to_base64(html: &str, base_path: &Path) -> String {
    let document = Document::from(html);

    document
        .find(Name("img"))
        .fold(html.to_string(), |mut acc, node| {
            if let Some(src) = node.attr("src") {
                // Determine if the src is a local relative path or a network URL
                let base64_data = if is_relative_path(src) {
                    // Local image: relative path or absolute path
                    let image_path = if Path::new(src).is_absolute() {
                        PathBuf::from(src) // Absolute path, use directly
                    } else {
                        base_path.join(src) // Relative path, combine with the base path
                    };
                    read_local_image_as_base64(&image_path)
                } else {
                    // Network image
                    download_image_as_base64(src)
                };

                // Replace the src attribute of <img> with Base64
                if let Some(base64_data) = base64_data {
                    acc = acc.replace(src, &base64_data);
                }
            }
            acc
        })
}

fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the HTML file is provided in arguments
    if args.len() < 3 {
        eprintln!("{} <currentPath> <outputPath>", pkg_name);
        return;
    }

    // Get the path to the Markdown file
    let markdown_file_path = PathBuf::from(&args[1]);

    // Get the path to the HTML file
    let html_file_path = PathBuf::from(&args[2]);

    // Read the content of the HTML file
    let html_content = fs::read_to_string(&html_file_path).expect("Unable to read the HTML file");

    // Set the base path to the directory where the Markdown file is located
    let base_path = markdown_file_path
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Convert all <img> tags' src to Base64
    let updated_html = convert_img_to_base64(&html_content, base_path);

    // Overwrite and save the modified HTML file
    let mut file = File::create(&html_file_path).expect("Unable to open HTML file for writing");
    file.write_all(updated_html.as_bytes())
        .expect("Unable to write to the modified HTML file");

    println!("Successfully converted all images to Base64 and saved the file.");
}
