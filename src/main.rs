use std::process::exit;
use std::{env};

mod color_palletting;

fn print_usage_and_exit() {
    println!("Usage: snes_image_encoder -i <input file> -o <output file>");
    println!("Too few arguments");
    exit(-1);
}

fn main() -> Result<(), i8> {

    let mut input_file = String::from("");
    let mut output_file = String::from("");

    let args: Vec<String> = env::args().collect();

    if args.len() < 5{
        print_usage_and_exit();
    }

    for i in 0..args.len() {
        if args[i] == "-i"{
            input_file = args[i+1].to_string();
        }
        else if args[i] == "-o" {
            output_file = args[i+1].to_string();
        }
    }

    println!("file path is: {}", output_file);
    let img = bmp::open(&input_file).unwrap_or_else(|e| {
        println!("Failed to open: {}", e);
        println!("Check if the path actually exists");
        exit(-1);
    });

    //Create color pallette
    println!("Creating color pallette");
    
    //Create color pallette and then write this pallette to file
    let mut pallette_colors: Vec<u16> = Vec::new();
    color_palletting::create_pallette(&img, &mut pallette_colors);
    
    if pallette_colors.len() == 0{
        println!("Something went wrong while trying to create the color pallette");
        exit(-1);
    }
    else {
        println!("Writing color pallette to {}", output_file);
        color_palletting::write_pallette(&output_file, &pallette_colors);
    }

    println!("Done");

    return Ok(());
} 