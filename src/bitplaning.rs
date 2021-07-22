use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write_snes_sprite(output_path: &str, indices: &Vec<u8>, bpp: u8, stride: usize){

    //Create and initialize bitplanes
    let mut bitplanes: Vec<Vec<u8>> = Vec::new();
    //For every bit per pixel, create and add a bitplane
    for bit in 0..bpp{
        //Since bitplanes are always 8x8, the size is 64 (8x8).
        bitplanes.push(vec![0; 64]);
    }


    //TODO: Do this for every tile
    //Every tile has a dimension of 8x8.
    setup_bitplanes(indices, &mut bitplanes, stride);
    transform_bitplanes(&mut bitplanes);
    write_bitplanes(output_path, &bitplanes);
}

//Sets up the bitplanes according to the given input indices.
fn setup_bitplanes(indices: &Vec<u8>, bitplanes: &mut Vec< Vec<u8> >, stride: usize){

    //Set corresponding bits of the bitplanes.
    //TODO:This need to be done for every 8x8 tile
    //Iterate over every index and set the corresponding bits of the Bitplanes
    for tile_row in 0..8{
        for tile_col in 0..8{
            //Take corresponding index from index vector
            let curr_idx = indices[tile_row*stride+tile_col];
            //Set corresponding bits in bitplanes
            for bp_num in 0..bitplanes.len(){
                //Set the bit on the corresponding column
                let p_curr_bitplane = &mut bitplanes[bp_num];
                p_curr_bitplane[tile_row*8+tile_col] = (curr_idx >> bp_num) & 0x01;
            }
        }
    }
}

//Write current bit planes out and clean the bitplanes after that
fn write_bitplanes(output_path: &str, bitplanes: &Vec< Vec<u8> >){
    let filepath = Path::new(output_path);
    let mut file = match OpenOptions::new()
                    .create(true)
                    .append(true).
                    open(output_path) {

                    Err(why) => panic!("Could not create {}: {}", filepath.display(), why),
                    Ok(file) => file
    };
    // let mut file  = match File::create(&filepath){
    //     Err(why) => panic!("Could not create {}: {}", filepath.display(), why),
    //     Ok(file) => file
    // };

    //Print every byte of the bitplanes in the right order
    //Order here is: intertwined bp0 and bp1...
    //...and after that intertwined bp2 and bp3
    let bp0 = bitplanes.get(0).unwrap();
    let bp1 = bitplanes.get(1).unwrap();
    let bp2 = bitplanes.get(2).unwrap();
    let bp3 = bitplanes.get(3).unwrap();

    let mut buffer: [u8; 2] = [0; 2];

    for row in 0..8{
        buffer[0] = bp0[row*8];
        buffer[1] = bp1[row*8];
        match file.write_all(&buffer){
            Err(why) => panic!("Could not write the bitplanes to {}: {}", output_path, why),
            Ok(_) => ()
        };
    }

    for row in 0..8{
        buffer[0] = bp2[row*8];
        buffer[1] = bp3[row*8];
        match file.write_all(&buffer){
            Err(why) => panic!("Could not write the bitplanes to {}: {}", output_path, why),
            Ok(_) => ()
        };
    }

    println!("Finished writing the bitplanes");
}

// This takes the firt byte of every row and sets the first byte...
// ...according to the byte representation of the current row.
// Does this via in place manipulation, which saves memory.
fn transform_bitplanes(bitplanes: &mut Vec< Vec<u8> >){

    for bp_num in  0..bitplanes.len(){
        let p_curr_bp = bitplanes.get_mut(bp_num).unwrap();
        for curr_row in 0..8{

            let mut final_byte: u8 = 0x00;

            for curr_col in 0..8{
                final_byte = final_byte << 1;
                final_byte = p_curr_bp[curr_row*8+curr_col]  ^ final_byte;
            }

            p_curr_bp[curr_row*8] = final_byte;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_bitplaning(){
        //Setup input indices and the expected output values of the bitplanes

        //For the indices, currently only the first tile is relevant.
        //For this reason the other index tiles, which are given here, ...
        //...are currently not relevant, since we only do tests for the first tile.
        //Other tiles are currently just fillers to make the test run.
        let input_indices = vec![
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,2,2,2,2,2,2,    1,1,1,1,1,1,1,1,
            1,1,2,2,2,2,2,2,    1,1,1,1,1,1,1,1,
            1,1,2,2,2,3,3,3,    1,1,1,1,1,1,1,1,
            1,1,2,2,2,3,3,3,    1,1,1,1,1,1,1,1,
            1,1,2,2,2,3,3,3,    1,1,1,1,1,1,1,1,
            1,1,2,2,2,3,3,4,     1,1,1,1,1,1,1,1,

            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,    1,1,1,1,1,1,1,1
        ];

        let mut exptected_bitplanes: Vec< Vec<u8> > = Vec::new();

        let bp0 = vec![
            1,1,1,1,1,1,1,1,
            1,1,1,1,1,1,1,1,
            1,1,0,0,0,0,0,0,
            1,1,0,0,0,0,0,0,
            1,1,0,0,0,1,1,1,
            1,1,0,0,0,1,1,1,
            1,1,0,0,0,1,1,1,
            1,1,0,0,0,1,1,0,
        ];

        let bp1 = vec![
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,1,1,1,1,1,1,
            0,0,1,1,1,1,1,1,
            0,0,1,1,1,1,1,1,
            0,0,1,1,1,1,1,1,
            0,0,1,1,1,1,1,1,
            0,0,1,1,1,1,1,0,
        ];

        let bp2 = vec![
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,1,
        ];

        let bp3 = vec![
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
        ];

        exptected_bitplanes.push(bp0);
        exptected_bitplanes.push(bp1);
        exptected_bitplanes.push(bp2);
        exptected_bitplanes.push(bp3);


        //Setup test bitplanes.
        //Currently only supporting 4bpp.
        let bpp = 4;
        let mut test_bitplanes: Vec<Vec<u8>> = Vec::new();
        for bit in 0..bpp{
            test_bitplanes.push(vec![0; 64]);
        }

        //Assuming a test image stride of 16.
        setup_bitplanes(&input_indices, &mut test_bitplanes, 16);

        //Check if the output (bitplanes) are as expected.
        for bp_num in 0..bpp{
            println!("Testing setup of BP#{}...", bp_num);
            assert_eq!(exptected_bitplanes[bp_num], test_bitplanes[bp_num]);
            println!("...Success")
        }

        //Currently this will only check on 4bpp, which currently is the only supported format anyway.
        //TODO: Check if bitplanes are transformed correctly.
        let bp0_bytes: [u8; 8] = [0xff, 0xff, 0xc0, 0xc0, 0xc7, 0xc7, 0xc7, 0xc6];
        let bp1_bytes: [u8; 8] = [0x0, 0x0, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3e];
        let bp2_bytes: [u8; 8] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01];
        let bp3_bytes: [u8; 8] = [0; 8];

        transform_bitplanes(&mut test_bitplanes);

        //Test bp0
        for row in 0..8 {
            let curr_bp = test_bitplanes.get(0).unwrap();
            assert_eq!(curr_bp[row*8], bp0_bytes[row]);
        }
        //Test bp1
        for row in 0..8 {
            let curr_bp = test_bitplanes.get(1).unwrap();
            assert_eq!(curr_bp[row*8], bp1_bytes[row]);
        }
        //Test bp2
        for row in 0..8 {
            let curr_bp = test_bitplanes.get(2).unwrap();
            assert_eq!(curr_bp[row*8], bp2_bytes[row]);
        }

        //Test bp3
        for row in 0..8 {
            let curr_bp = test_bitplanes.get(3).unwrap();
            assert_eq!(curr_bp[row*8], bp3_bytes[row]);
        }
    }
}