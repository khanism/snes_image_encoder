use bmp::{Image, Pixel};
use std::path::Path;
use std::process::exit;
use std::{env};
use std::fs::File;
use std::io::prelude::*;

fn print_usage_and_exit() {
    println!("Usage: snes_image_encoder -i <input file> -o <output file>");
    println!("Too few arguments");
    exit(-1);
}

pub trait OpenFields{
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

impl OpenFields for bmp::Pixel{
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
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
    create_pallette(&img, &mut pallette_colors);
    
    if pallette_colors.len() == 0{
        println!("Something went wrong while trying to create the color pallette");
        exit(-1);
    }
    else {
        println!("Writing color pallette to {}", output_file);
        write_pallette(&output_file, &pallette_colors);
    }

    println!("Done");

    return Ok(());
} 

//This will create a pallette and write the pallette file to the same location as the input sprite
//Also this will fill the given map with the colors
fn create_pallette(input_img: &Image, pallette_colors:  &mut Vec<u16>){
    
    //Iterate through the pixels in the input sprite
    //Fill pallette for each new color value
    for (x,y) in input_img.coordinates() {
        let p_rgb888 = input_img.get_pixel(x, y);
        let p_bgr555 = rgb888_to_bgr555(p_rgb888);
        
        //If color pallette already contains key, skip to next pixel..
        //..otherwise add this color to the pallette, using the snes format pallette color as key
        if !pallette_colors.contains(&p_bgr555) {
            pallette_colors.push(p_bgr555);
        }
    }
}

fn rgb888_to_bgr555(rgb888_input: Pixel) -> u16 {
    let mut bgr555_out: u16 = 0x0;

    //Set the 3 LSBs of every input color byte to 0
    //Also shift to right, so that the LSB zeros are shifted out
    let unset_r = unset_LSBs(rgb888_input.r) >> 3;
    let unset_g = unset_LSBs(rgb888_input.g) >> 3;
    let unset_b = unset_LSBs(rgb888_input.b) >> 3;

    //Set correspoding bits of u16 output
    bgr555_out = bgr555_out ^ (unset_r as u16);
    bgr555_out = bgr555_out ^ ((unset_g as u16) << 5);
    bgr555_out = bgr555_out ^ ((unset_b as u16) << 10);
    
    return bgr555_out;
}

fn unset_LSBs(u8_input: u8) -> u8 {
    let tmp = 0xf8; //248, 11111|000
    u8_input & tmp
}

fn write_pallette(filepath: &str, color_pallette: &Vec<u16>){
    //Create file (overwrites if it already exists)
    let filepath = Path::new(filepath);
    let mut file = match File::create(&filepath) {
        Err(why) => panic!("Could not create {}: {}", filepath.display(), why),
        Ok(file) => file
    };

    //Write all bytes of the color pallette to file
    //Bytes of pallette must be written in little endian order

    let mut buffer: Vec<u8> = Vec::new();
    for idx in 0..color_pallette.len() {
        //Reverse order of u16 val
        let bgr555_color = color_pallette[idx];
        let lsb: u8 = 0xff & bgr555_color as u8;
        let msb: u8 = ((0xff00  & bgr555_color) >> 8) as u8;
        buffer.push(lsb);
        buffer.push(msb);
    }

    match file.write_all(&buffer) {
        Err(why) => panic!("Could not create {}: {}", filepath.display(), why),
        Ok(_) => println!("Finished writing the color pallette")
    };
}