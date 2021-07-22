use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use bmp::{Image, Pixel};
use std::convert::TryInto;

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

//This will create a pallette and write the pallette file to the same location as the input sprite
//Also this will fill the given map with the colors
pub fn create_pallette_and_indexing(input_img: &Image, pallette_colors:  &mut Vec<u16>, indices: &mut Vec<u8>){
    
    //Iterate through the pixels in the input sprite
    //Fill pallette for each new color value
    for (x,y) in input_img.coordinates() {

        //This is the corresponding index as a 1D index (maps 2D coord to 1D coord).
        let curr_idx:usize = (y*input_img.get_width() + x).try_into().unwrap();

        let p_rgb888 = input_img.get_pixel(x, y);
        let p_bgr555 = rgb888_to_bgr555(p_rgb888);
        
        //If color pallette already has 'seen' this pixel color, skip to next pixel..
        //..otherwise add this color to the pallette, using the snes format pallette color as key.
        //Since the first index is the alpha channel, always offset final index with 1.
        if !pallette_colors.contains(&p_bgr555) {
            pallette_colors.push(p_bgr555);
            indices[curr_idx] = ((pallette_colors.len()-1) as u8) + 1;
        } else {
            indices[curr_idx] = get_idx_of_color(pallette_colors, &p_bgr555) + 1;
        }
    }
}

fn rgb888_to_bgr555(rgb888_input: Pixel) -> u16 {
    let mut bgr555_out: u16 = 0x0;

    //Set the 3 LSBs of every input color byte to 0
    //Also shift to right, so that the LSB zeros are shifted out
    let unset_r = unset_lsbs(rgb888_input.r) >> 3;
    let unset_g = unset_lsbs(rgb888_input.g) >> 3;
    let unset_b = unset_lsbs(rgb888_input.b) >> 3;

    //Set correspoding bits of u16 output
    bgr555_out = bgr555_out ^ (unset_r as u16);
    bgr555_out = bgr555_out ^ ((unset_g as u16) << 5);
    bgr555_out = bgr555_out ^ ((unset_b as u16) << 10);
    
    return bgr555_out;
}

fn unset_lsbs(u8_input: u8) -> u8 {
    let tmp = 0xf8; //248, 11111|000
    u8_input & tmp
}

pub fn write_pallette(filepath: &str, color_pallette: &Vec<u16>){
    //Create file (overwrites if it already exists)
    let filepath = Path::new(filepath);
    let mut file = match File::create(&filepath) {
        Err(why) => panic!("Could not create {}: {}", filepath.display(), why),
        Ok(file) => file
    };

    //Write all bytes of the color pallette to the given file.
    //Bytes of pallette must be written in little endian order.

    //Use white (0xffff) as the alpha channel, which would be the first index of the pallette
    let mut buffer: Vec<u8> = vec![0xff, 0xff];

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

//Returns the index of a given color, inside a color_pallette.
//If there is no such color, this will panic, since that case should not happen
fn get_idx_of_color(pallette_colors: & Vec<u16>, color: &u16) -> u8{
    
    for idx in 0..pallette_colors.len() {
        if pallette_colors[idx] == *color{
            return idx as u8;
        }
    }
    panic!("Could not get index for a color (inside of get_idx_of_color)");

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_get_idx_of_color(){

        //Setup test values.
        let test_pallette: Vec<u16> = vec![0x0000, 0x1aed, 0x7e6c, 0x18d5];
        //Run test function.
        let result = get_idx_of_color(&test_pallette, &0x0000);
        assert_eq!(result, 0);
        let result = get_idx_of_color(&test_pallette, &0x1aed);
        assert_eq!(result, 1);
        let result = get_idx_of_color(&test_pallette, &0x7e6c);
        assert_eq!(result, 2);
        let result = get_idx_of_color(&test_pallette, &0x18d5);
        assert_eq!(result, 3);
    }
}

