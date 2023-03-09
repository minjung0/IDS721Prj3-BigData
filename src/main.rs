// a command-line interface (CLI) tool in Rust that can resize images in bulk
// The tool will take a directory path as input and recursively find all image files in the directory and its subdirectories.
// The tool will then resize all images to the specified width and height and save them to the output directory.

use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The path to the directory to read images from
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    // The width to resize the images to
    width: u32,

    // The height to resize the images to
    height: u32,

    // The path to the directory to write the resized images to
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    // Check if the input path is a directory
    if !args.path.is_dir() {
        println!("The path {:?} is not a directory", args.path);
        std::process::exit(1);
    }

    // Check if the output path is a directory
    if !args.output.is_dir() {
        println!("The path {:?} is not a directory", args.output);
        std::process::exit(1);
    }

    // Read the directory contents in a vector, sort the vector and iterate over it
    let entries = fs::read_dir(&args.path)
        .expect("read_dir call failed")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect("Failed to collect the results");

    for entry in entries {
        // Check if the entry is a file
        if entry.is_file() {
            // Check if the file is an image
            if is_image(&entry) {
                // Resize the image and save it to the output directory
                resize_image(&entry, &args.width, &args.height, &args.output);
            }
        }
    }
}

// Check if the file is an image
fn is_image(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("jpg") => true,
            Some("jpeg") => true,
            Some("png") => true,
            Some("gif") => true,
            Some("bmp") => true,
            Some("tiff") => true,
            Some("webp") => true,
            Some("ico") => true,
            Some("svg") => true,
            _ => false,
        },
        None => false,
    }
}

fn resize_image(path: &PathBuf, width: &u32, height: &u32, output: &PathBuf) {
    // Open the image
    let img = image::open(path).expect("Failed to open the image");

    // Resize the image
    let resized_img = img.resize(*width, *height, image::imageops::FilterType::Lanczos3);

    // Get the file name from the original path
    let file_name = path.file_name().unwrap().to_str().unwrap();

    // Save the resized image
    let mut output_path = output.clone();
    output_path.push(file_name);
    resized_img.save(output_path).expect("Failed to save the image");
}