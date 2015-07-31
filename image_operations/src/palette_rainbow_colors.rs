use pixel_image_simple::RGBA8;
use palette::PaletteRGBA8;

pub struct PaletteRainbowColorsRGBA8 {
    pub palette: Vec<RGBA8>,
}

impl PaletteRGBA8 for PaletteRainbowColorsRGBA8 {
    fn new() -> PaletteRainbowColorsRGBA8 {
        let mut palette = PaletteRainbowColorsRGBA8 { palette: vec![] };
        /*
        palette.palette.push(RGBA8 {r: 255, g: 0, b: 0, a: 255});
        palette.palette.push(RGBA8 {r: 255, g: 0, b: 28, a: 255});
        palette.palette.push(RGBA8 {r: 255, g: 0, b: 125, a: 255});
        palette.palette.push(RGBA8 {r: 255, g: 0, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 210, g: 0, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 150, g: 0, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 80, g: 0, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 0, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 60, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 140, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 200, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 255, b: 255, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 255, b: 200, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 255, b: 150, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 255, b: 100, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 255, b: 50, a: 255});
        palette.palette.push(RGBA8 {r: 0, g: 255, b: 0, a: 255});
        */

        /*
        let count_colors = 255;
        let increment = 1;
        let mut val = 255;

        for i in 0..count_colors {
            palette.palette.push(RGBA8 {r: val, g: 0, b: 0, a: 255});
            println!("PaletteRainbowColorsRGBA8   - i: {}, val: {}",
                i, val);
            if (val >= increment) {
                val = val - increment;
            } else {
                val = 0;
            }
        }
        */
        let count_colors = 16;
        let increment = 256 / count_colors;
        let mut val = 255;
        for i in 0..count_colors {
            palette.palette.push(RGBA8 {r: 255, g: 0, b: val, a: 255});
            println!("PaletteRainbowColorsRGBA8   - i: {}, val: {}",
                i, val);
            if (val >= increment) {
                val = val - increment;
            } else {
                val = 0;
            }
        }
        val = 255;
        for i in 0..count_colors {
            palette.palette.push(RGBA8 {r: val, g: 0, b: 255, a: 255});
            if (val >= increment) {
                val = val - increment;
            } else {
                val = 0;
            }
        }
        val = 0;
        for i in 0..count_colors {
            palette.palette.push(RGBA8 {r: 0, g: val, b: 255, a: 255});
            if (val <= 255-increment) {
                val = val + increment;
            } else {
                val = 255;
            }
        }
        val = 0;
        for i in 0..count_colors {
            palette.palette.push(RGBA8 {r: 0, g: 255, b: val, a: 255});
            if (val >= increment) {
                val = val - increment;
            } else {
                val = 0;
            }
        }
        palette
    }

    fn get_palette(self) -> PaletteRainbowColorsRGBA8 {
         self
    }

    fn get_palette_length(&self) -> usize {
         self.palette.len()
    }
}
