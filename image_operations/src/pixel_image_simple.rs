extern crate lodepng;
extern crate image as png_image;

use std::path::Path;
use std::fs::File;

pub struct PixelImageSimple {
    pub pixels: Vec<i32>,
    pub width: i32,
    pub height: i32,
}

impl PixelImageSimple  {
    pub fn new() -> PixelImageSimple {
        PixelImageSimple { pixels: vec![], width: 0, height:0 }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> i32 {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, val: i32) {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx] = val;
    }

    pub fn save_png(self, filename: &str) {
        let path = &Path::new(filename);

        let mut image: Vec<u8> = vec![0; ((self.width * self.height)*3) as usize];

        let mut idx = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get_pixel(x, y) as u8;
                 image[idx] = val;
                 image[idx+1] = val;
                 image[idx+2] = val;
                 idx += 3;
            }
        }

        println!("save_png:  width: {}, height: {}", self.width, self.height);
        // encode_file takes the path to the image, a u8 array,
        // the width, the height, the color mode, and the bit depth

        if let Err(e) = lodepng::encode_file(path, &image, self.width as usize, self.height as usize, lodepng::LCT_RGB, 8) {
            panic!("failed to write png: {:?}", e);
        }
    }

    pub fn save_png2(self, filename: &str) {
        let path = &Path::new(filename);

        let mut imgbuf = png_image::ImageBuffer::new(self.width as u32, self.height  as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let i = self.get_pixel(x as i32, y as i32);
            *pixel = png_image::Luma([i as u8]);
        }
        let ref mut fout = File::create(path).unwrap();
        let _ = png_image::ImageLuma8(imgbuf).save(fout, png_image::PNG);
    }
}

pub struct ImageOperationParam {
    pub startx: i32,
    pub starty: i32,
    pub endx: i32,
    pub endy: i32,
    pub bitmap: PixelImageSimple
}

impl ImageOperationParam  {
    pub fn new() -> ImageOperationParam {
        let emtpy =  PixelImageSimple { pixels: vec![], width: 0, height:0 };
        ImageOperationParam { bitmap: emtpy, startx: 0, starty: 0, endx: 0, endy: 0 }
    }

    pub fn new2(width: i32, height: i32, bitmapdata: Vec<i32>) -> ImageOperationParam {
        let emtpy =  PixelImageSimple { pixels: bitmapdata, width: width, height: height };
        ImageOperationParam { bitmap: emtpy, startx: 0, starty: 0, endx: 0, endy: 0 }
    }
}
