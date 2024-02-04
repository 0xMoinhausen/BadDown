use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{Result, Context};
use clap::Parser as CliParser;
use parser::Parser as BdParser;

#[derive(CliParser, Debug)]
#[clap(author = "Moinhausen", version, about)]
/// A very simple Baddown to HTML Converter
struct Arguments {
    bd_file_path: PathBuf,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Arguments::parse();

    // Extract file stem from the provided path
    let file_stem = args.bd_file_path
        .file_stem()
        .with_context(|| "Invalid file path, expecting a file that ends with .bd")?
        .to_string_lossy()
        .to_string();

    // Read Baddown file content
    let file_content = fs::read_to_string(&args.bd_file_path)
        .with_context(|| format!("Failed to read content from {:?}", args.bd_file_path))?;

    // Parse Baddown to HTML
    let html = BdParser::parse(file_content.chars().peekable(), file_stem.clone());

    // Create HTML file with the same name as Baddown file but with .html extension
    let html_file_path = format!("{}.html", file_stem);
    let mut html_file = File::create(&html_file_path)
        .with_context(|| format!("Failed to create HTML file at {:?}", html_file_path))?;

    // Write HTML content to the file
    html_file.write_all(html.as_string().as_bytes())
        .with_context(|| format!("Failed to write HTML content to {:?}", html_file_path))?;

    println!("Conversion successful! HTML file created at: {}", html_file_path);
    Ok(())
}