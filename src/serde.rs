use crate::paint;
use crate::paint::Grid;
use crate::paint::Palette;

/// Returns: VRAM data (ret.0) and Palette data (ret.1). Colors stored little-endian (SNES specs)
pub fn write_out(grid: &dyn Grid<usize>, palette: &Palette) -> (Vec<u8>, Vec<u8>) {
    let mut v_ram = vec![];
    let mut pal = vec![];
    match palette.bpp() {
        2 => {
            // iter over index chunks of 8x8, left to right, up to down
            let num_sprite_width = grid.width() / 8;
            let num_sprite_height = grid.height() / 8;
            for i in 0..num_sprite_width {
                for j in 0..num_sprite_height {
                    let subgrid = paint::subgrid::<8, 8>(grid, (i*8, (i+1)*8), (j*8, (j+1)*8));
                    for chunk in 0..subgrid.width() * subgrid.height() / 8 {
                        // intertwine two bit planes:
                        let mut bp1 = 0u8;
                        let mut bp2 = 0u8;
                        // for each item in the subgrid...
                        for i in chunk*8..(chunk+1)*8 {
                            let v = subgrid.idx_linear(i);
                            // store first bit in bp1
                            bp1 <<= 1;
                            // push for next fella
                            bp1 |= (v as u8 & 0b0001) >> 0;
                            // store second bit in bp2
                            bp2 <<= 1;
                            // push for next fella
                            bp2 |= (v as u8 & 0b0010) >> 1;
                        }
                        // add to array
                        v_ram.push(bp1);
                        v_ram.push(bp2);
                    }
                    v_ram.extend_from_slice(&[0u8;16]);
                }
            }


        }
        3 => {
            unimplemented!()
        }
        4 => {
            unimplemented!()
        }
        8 => {
            unimplemented!()
        }
        _ => {
            panic!("Bad bpp mode {}!!", palette.bpp());
        }
    }

    for c in 0..palette.size() {
        let color = palette[c];
        let bytes: u16 = {
            (color.b() as u16 >> 3) << 10 | (color.g() as u16 >> 3) << 5 | (color.r() as u16 >> 3)
        };
        let ls_byte = (bytes & 0x00ff) as u8;
        let ms_byte = ((bytes & 0xff00) >> 8) as u8;
        pal.push(ls_byte);
        pal.push(ms_byte);
    }

    (v_ram, pal)
}