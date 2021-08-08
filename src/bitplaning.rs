use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;


pub fn write_snes_sprite(output_path: &str, indices: &Vec<u8>, bpp: u8, stride: usize){

    //Create and initialize bitplanes
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    //For every bit per pixel, create and add a tile for the bitplanes.
    for _bit in 0..bpp{
        // Since bitplanes are always 8x8, the size is 64 (8x8).
        // NOTE: Assuming that the sprite height and width is equal.
        // Every entry here will contain all bitplanes for one indices tile.
        tiles.push(vec![0; stride*stride]);
    }


    // Do this for every tile.
    // Indices and color pallette is already set for the full sprite image.
    // The setup of the bitplanes must be done for all tiles.
    // TODO: After the bitplanes are written, the bitplane entries must be resetted (set to 0), for the next tile.
    // Only supporting 16x16 Sprites.

    // Every tile has a dimension of 8x8. So for 16x16 there are 4 tiles.
    for tile_num in 0..4 {
        setup_bitplanes(indices, &mut tiles, stride, tile_num);
        transform_bitplanes(tiles.get_mut(0).unwrap() , stride); 
        write_bitplanes(output_path, &tiles, stride, tile_num);
    }

    println!(" --- Finished writing sprite bitplanes ----");
}

// Sets up the bitplanes according to the given input indices.
// This sets the corresponding bytes on the bitplanes
// for every tile in the given indices.
fn setup_bitplanes(indices: &Vec<u8>, bitplanes: &mut Vec< Vec<u8> >, stride: usize, tile_num: usize){

    //Set corresponding bits of the bitplanes.
    //Done for every 8x8 tile.
    //Iterate over every index and set the corresponding bits of the Bitplanes.

    let col_offs_bit = (tile_num & 0x01) as usize;
    let row_offs_bit = ((tile_num & 0x02) >> 1) as usize;

    for tile_row in (row_offs_bit*8)..(row_offs_bit*8+8){
        for tile_col in (col_offs_bit*8)..(col_offs_bit*8+8){
            //Take corresponding index from index vector
            let curr_idx = indices[tile_row*stride+tile_col];
            //Set corresponding bits in bitplanes
            for bp_num in 0..bitplanes.len(){
                //Set the bit on the corresponding column
                let p_curr_tile = &mut bitplanes[tile_num];
                //TODO: set correct tile bits according to current BP
                let bp_col_offs_bit = (bp_num & 0x01) as usize;
                let bp_row_offs_bit = ((bp_num & 0x02) >> 1) as usize;
                let final_idx = (bp_row_offs_bit*8*stride + ((tile_row%8)*stride)) + (bp_col_offs_bit*8 + (tile_col%8));

                p_curr_tile[final_idx] = (curr_idx >> bp_num) & 0x01;
            }
        }
    }
}

//Write current bit planes out.
//Doing that for every tile.
fn write_bitplanes(output_path: &str, bitplanes: &Vec< Vec<u8> >, stride: usize, tile_num: usize){
    let filepath = Path::new(output_path);
    let mut file = match OpenOptions::new()
                    .create(true)
                    .append(true).
                    open(output_path) {

                    Err(why) => panic!("Could not create {}: {}", filepath.display(), why),
                    Ok(file) => file
    };

    //Print every byte of the bitplanes in the right order
    //Order here is: intertwined bp0 and bp1...
    //...and after that intertwined bp2 and bp3
    let bp0 = bitplanes.get(0).unwrap();
    let bp1 = bitplanes.get(1).unwrap();
    let bp2 = bitplanes.get(2).unwrap();
    let bp3 = bitplanes.get(3).unwrap();

    let mut buffer: [u8; 2] = [0; 2];

    let row_offs_bit = ((tile_num & 0x02) >> 1) as usize;
    
    for row in (row_offs_bit*8)..(row_offs_bit*8+8){

        //Write bp0 and bp1
        buffer[0] = bp0[row*stride];
        buffer[1] = bp1[row*stride];
        match file.write_all(&buffer){
            Err(why) => panic!("Could not write the bitplanes to {}: {}", output_path, why),
            Ok(_) => ()
        };

        //TODO: Optimize with only one write call (instead calling write operation mutiple times).

        //Write bp2 and bp3
        buffer[0] = bp2[row*stride];
        buffer[1] = bp3[row*stride];
        match file.write_all(&buffer){
            Err(why) => panic!("Could not write the bitplanes to {}: {}", output_path, why),
            Ok(_) => ()
        };

    }
}

// This takes the firt byte of every row and sets that byte...
// ...according to the byte representation of the current row.
// Does this via in place manipulation, which saves memory.
fn transform_bitplanes(bitplanes: &mut Vec< u8 >, stride: usize){

    for bp_num in 0..4{

        let bp_col_offs_bit = (bp_num & 0x01) as usize;
        let bp_row_offs_bit = ((bp_num & 0x02) >> 1) as usize;
        

        for curr_row in (bp_row_offs_bit*8)..(bp_row_offs_bit*8 + 8){

            let mut final_byte: u8 = 0x00;

            for curr_col in (bp_col_offs_bit*8)..(bp_col_offs_bit*8+8){
                final_byte = final_byte << 1;
                final_byte = bitplanes[curr_row*stride + curr_col]  ^ final_byte;
            }

            bitplanes[curr_row*stride+bp_col_offs_bit*8] = final_byte;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_bitplaning(){

        //Setup input indices and the expected output values of the bitplanes

        //Test indices, which make up the full test image (test_sprite.bmp)
        //Only the first tile (top left) is correct, which is all we need for this test
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

        //Since we only test the first tile
        //we onl got the bitplanes for the first tile here
        let exptected_bitplanes: Vec< Vec<u8> > = vec![vec![
            1,1,1,1,1,1,1,1, 0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,1,1, 0,0,0,0,0,0,0,0,
            1,1,0,0,0,0,0,0, 0,0,1,1,1,1,1,1,
            1,1,0,0,0,0,0,0, 0,0,1,1,1,1,1,1,
            1,1,0,0,0,1,1,1, 0,0,1,1,1,1,1,1,
            1,1,0,0,0,1,1,1, 0,0,1,1,1,1,1,1,
            1,1,0,0,0,1,1,1, 0,0,1,1,1,1,1,1,
            1,1,0,0,0,1,1,0, 0,0,1,1,1,1,1,0,

            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,1, 0,0,0,0,0,0,0,0
        ]];

        //Setup test bitplanes
        //Currently only supporting 4bpp and 16x16 sprites
        let bpp = 4;
        let mut test_bitplanes: Vec<Vec<u8>> = Vec::new();
        for _bit in 0..bpp{
            test_bitplanes.push(vec![0; 16*16]);
        }

        //Assuming a test image stride of 16 (since sprites are assumed to be 16x16).
        //Testing only the first tile (upper left tile).
        setup_bitplanes(&input_indices, &mut test_bitplanes, 16, 0);

        //Check if the output (bitplanes) are as expected.
        assert_eq!(exptected_bitplanes[0], test_bitplanes[0]);

        //Currently this will only check on 4bpp, which currently is the only supported format anyway.
        //TODO: Check if bitplanes are transformed correctly.
        let bp0_bytes: [u8; 8] = [0xff, 0xff, 0xc0, 0xc0, 0xc7, 0xc7, 0xc7, 0xc6];
        let bp1_bytes: [u8; 8] = [0x0, 0x0, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3e];
        let bp2_bytes: [u8; 8] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01];
        let bp3_bytes: [u8; 8] = [0; 8];

        transform_bitplanes(test_bitplanes.get_mut(0).unwrap(), 16);
        
        let tile0 = test_bitplanes.get(0).unwrap();

        //Test bp0
        for idx in 0..8 {
            assert_eq!(tile0[idx * 16], bp0_bytes[idx]);
        }


        //Test bp1
        for idx in 0..8 {
            assert_eq!(tile0[idx * 16 + 8], bp1_bytes[idx]);
        }

        //Test bp2
        for idx in 0..8 {
            assert_eq!(tile0[(8*16) + (idx *16)], bp2_bytes[idx]);
        }


        //Test bp3
        for idx in 0..8 {
            assert_eq!(tile0[(8*16) + (idx *16) + 8], bp3_bytes[idx]);
        }
    }
}