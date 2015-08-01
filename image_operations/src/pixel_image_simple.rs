extern crate lodepng;

use std::path::Path;
use std::f64;

use palette_rainbow_colors::PaletteRainbowColorsRGBA8;

#[derive(Copy,Clone)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub struct PixelImageSimple {
    pub pixels: Vec<RGBA8>,
    pub width: i32,
    pub height: i32,
}

fn modulo(a: i32, b: i32) -> i32 {
    let dummy = (a as f64 / b as f64).floor() as i32;
    //println!("{} /  {} = {}", a, b, dummy);
    let bla = a - dummy * b;
    //println!("{} -  {} * {} = {}", a, b, dummy, bla);
    bla
}

impl PixelImageSimple  {
    pub fn new() -> PixelImageSimple {
        PixelImageSimple { pixels: vec![], width: 0, height:0 }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> RGBA8 {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, val: RGBA8) {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx] = val;
    }

    pub fn save_png(&self, filename: &str) {
        let path = &Path::new(filename);

        let mut image: Vec<u8> = vec![0; ((self.width * self.height)*3) as usize];

        let mut idx = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get_pixel(x, y);
                //println!("save_png   - x: {}, y: {}, val  r: {}, g: {}, b: {}, a: {}",
                //    x,y , val.r, val.g, val.b, val.a);

                image[idx] = val.r;
                image[idx+1] = val.g;
                image[idx+2] = val.b;
                // image[idx+3] = val.a;
                // idx += 4;
                idx += 3;
            }
        }

        for i in 0..image.len()  {
            //println!("val  image[{}]=    image: {} ", i, image[i]);
        }
         // encode_file takes the path to the image, a u8 array,
        // the width, the height, the color mode, and the bit depth

        if let Err(e) = lodepng::encode_file(path, &image, self.width as usize, self.height as usize, lodepng::LCT_RGB, 8) {
            panic!("failed to write png: {:?}", e);
        }
    }

    pub fn create_fractal(&mut self, palette: PaletteRainbowColorsRGBA8) {
        // pixel size of the fractal
        let topleftx: f64 = -1.5;
        let toplefty: f64 = 1.0;
        let bottomrightx: f64 = 0.8;
        let bottomrighty: f64 = -1.0;

        let incrementx = (topleftx.abs() + bottomrightx.abs()) / self.width as f64;
        let incrementy = (toplefty.abs() + bottomrighty.abs()) / self.height as f64;

        let mut px: f64 = topleftx;
        let mut py: f64 = toplefty;
        //let mut real = 0.0;
        //let mut imag = 0.0;
        let mut count_iteration = 0;
        let max_iteration = 500;
        let max_distance = 5.0;
        let mut distance = 0.0;

        let mut pixel: RGBA8 = RGBA8 {r:0, g: 0, b: 0, a: 0};

        // TODO: why does this not work?
        // let count_colors = palette.get_palette_length();
        // but this does?
        let count_colors = palette.palette.len();
        println!("create_fractal:  count_colors: {}", count_colors);
        //println!("create_fractal:  width: {}, height: {}",
        //    self.width, self.height );

        //println!("create_fractal:  incrementx: {}, incrementy: {}",
        //    incrementx, incrementy );

        let mut idx_palette = 0;

        let mut real = 0.0;
        let mut imag = 0.0;
        let mut a = 0.0;
        let mut b = 0.0;
        let mut dummy: f64 = 0.0;

        for iter_x in 0..self.width {
            for iter_y in 0..self.height {
                px = topleftx + incrementx * iter_x as f64;
                py = toplefty - incrementy * iter_y as f64;
                count_iteration = 0;
                distance = 0.0;
                a = 0.0;
                b = 0.0;
                while (count_iteration <= max_iteration) && (distance <= (max_distance*max_distance)) {
                    real = a * a - b * b + px;
                    imag = 2.0 * a * b + py;
                    distance = real * real + imag * imag;
                    count_iteration += 1;

                    //println!("create_fractal: iter_x: {}, iter_y: {},  px: {}, py: {}, real: {}, imag: {}, distance {},  count_iteration: {}",
                    //     iter_x, iter_y, px, py, real, imag, distance, count_iteration);
                    a = real;
                    b = imag;
                }
                idx_palette = modulo(count_iteration, count_colors as i32) as usize;

                //println!("create_fractal:  count_iteration: {}, count_colors: {}, distance {},  max_distance: {},  idx_palette: {}, dummy: {}",
                //   count_iteration, count_colors, distance, max_distance*max_distance, idx_palette, dummy);

                pixel = palette.palette[idx_palette];

                self.set_pixel(iter_x, iter_y, pixel);

                //println!("create_fractal:  iter_x: {}, iter_y: {}",
                //    iter_x, iter_y );
            }
        }
    }

    pub fn create_white_square(&mut self) {
        // pixel size of the fractal
        for y in 0..self.height {
            for x in 0..self.width  {
                let pixel = RGBA8 {r: 0, g: 0, b: 0, a: 255 };
                self.set_pixel(x, y, pixel);
            }
        }

        let square_width = 40;
        let startx = self.width / 2 - square_width / 2;
        let starty = self.height / 2 - square_width / 2;

        for y in starty..(starty + square_width) {
            for x in startx..(startx + square_width) {
                let pixel = RGBA8 {r: 255, g: 255, b: 255, a: 255 };
                self.set_pixel(x, y, pixel);
            }
        }
    }

    pub fn create_white_rectangle(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width  {
                let pixel = RGBA8 {r: 0, g: 0, b: 0, a: 255 };
                self.set_pixel(x, y, pixel);
            }
        }

        let square_width = 80;
        let square_height = 10;

        let startx = self.width / 2 - square_width / 2;
        let starty = self.height / 2 - square_height / 2;

        for y in starty..(starty + square_height) {
            for x in startx..(startx + square_width) {
                let pixel = RGBA8 {r: 255, g: 255, b: 255, a: 255 };
                self.set_pixel(x, y, pixel);
            }
        }
    }

    pub fn create_stripes(&mut self, stripe_width: i32) {
        // pixel size of the fractal
        for y in 0..self.height {
            for x in 0..self.width  {
                let pixel = RGBA8 {r: 0, g: 0, b: 0, a: 255 };
                self.set_pixel(x, y, pixel);
            }
        }

       for x in (0..self.width).step_by(stripe_width) {
            println!("create_stripes: x = {}", x);
            for y in 0..self.height {
                let pixel = RGBA8 {r: 255, g: 255, b: 255, a: 255 };
                self.set_pixel(x, y, pixel);
            }
        }
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

    pub fn new2(width: i32, height: i32, bitmapdata: Vec<RGBA8>) -> ImageOperationParam {
        let emtpy =  PixelImageSimple { pixels: bitmapdata, width: width, height: height };
        ImageOperationParam { bitmap: emtpy, startx: 0, starty: 0, endx: 0, endy: 0 }
    }
}
