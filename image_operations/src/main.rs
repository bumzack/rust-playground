#![feature(step_by)]

extern crate num;

use std::rc::Rc;

use std::f64;
use std::f64::consts;

use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;
use pixel_image_simple::RGBA8;

use image_operation::ImageOperation;
use image_operation_fft::ImageOperationFFT;

use image::Image;

use palette::PaletteRGBA8;
use palette_rainbow_colors::PaletteRainbowColorsRGBA8;

mod pixel_image_simple;
mod image_operation;
mod image_operation_fft;
mod palette;
mod palette_rainbow_colors;
mod image;

fn create_stripes() {
    let width = 256;
    let height = 256;
    let size = width * height;

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];

    let mut stripes_bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };
    stripes_bitmap.create_stripes(2);
    stripes_bitmap.save_png("stripes_original.png");

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];
    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let mut fft = ImageOperationFFT { input_bitmapdata: bitmap, output_bitmapdata: PixelImageSimple { pixels: bitmapdata, width: width, height: height } };

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];

    fft.set_input_bitmap(stripes_bitmap);

    let res_bitmap: Vec<RGBA8> = vec![];
    let mut input = ImageOperationParam::new2(0, 0, res_bitmap.clone());
    let mut bla = ImageOperationParam::new2(0, 0, res_bitmap);

    bla = fft.execute_op2(&input);

    let mut res2 = PixelImageSimple { pixels: vec![], width: 0, height: 0 };

    let res2 = fft.get_output_bitmap();

    res2.save_png("stripes_transformed.png");
}

fn white_square() {
    let width = 256;
    let height = 256;
    let size = width * height;

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];

    let mut square_bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };
    square_bitmap.create_white_square();
    square_bitmap.save_png("square_original.png");

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];
    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let mut fft = ImageOperationFFT { input_bitmapdata: bitmap, output_bitmapdata: PixelImageSimple { pixels: bitmapdata, width: width, height: height } };

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let fractal = PixelImageSimple { pixels: bitmapdata, width: width, height: height };

    fft.set_input_bitmap(square_bitmap);

    let res_bitmap: Vec<RGBA8> = vec![];
    let mut input = ImageOperationParam::new2(0, 0, res_bitmap.clone());
    let mut bla = ImageOperationParam::new2(0, 0, res_bitmap);

    bla = fft.execute_op2(&input);

    let mut res2 = PixelImageSimple { pixels: vec![], width: 0, height: 0 };

    let res2 = fft.get_output_bitmap();

    res2.save_png("square_square_transformed.png");
}


fn white_rectangle() {
    let width = 256;
    let height = 256;
    let size = width * height;

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];

    let mut rect_bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };
    rect_bitmap.create_white_rectangle();
    rect_bitmap.save_png("rect_original.png");

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];
    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let mut fft = ImageOperationFFT { input_bitmapdata: bitmap, output_bitmapdata: PixelImageSimple { pixels: bitmapdata, width: width, height: height } };

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];

    fft.set_input_bitmap(rect_bitmap);

    let res_bitmap: Vec<RGBA8> = vec![];
    let mut input = ImageOperationParam::new2(0, 0, res_bitmap.clone());
    let mut bla = ImageOperationParam::new2(0, 0, res_bitmap);

    bla = fft.execute_op2(&input);

    let mut res2 = PixelImageSimple { pixels: vec![], width: 0, height: 0 };

    let res2 = fft.get_output_bitmap();

    res2.save_png("rect_transformed.png");
}

fn fractal() {
    let width = 256;
    let height = 256;
    let size = width * height;

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];
    let palette = PaletteRainbowColorsRGBA8::new();

    let mut factal_bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };
    factal_bitmap.create_fractal(palette);
    factal_bitmap.save_png("fractal_fft_original.png");

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];
    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let mut fft = ImageOperationFFT { input_bitmapdata: bitmap, output_bitmapdata: PixelImageSimple { pixels: bitmapdata, width: width, height: height } };

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let fractal = PixelImageSimple { pixels: bitmapdata, width: width, height: height };

    fft.set_input_bitmap(factal_bitmap);

    let res_bitmap: Vec<RGBA8> = vec![];
    let mut input = ImageOperationParam::new2(0, 0, res_bitmap.clone());
    let mut bla = ImageOperationParam::new2(0, 0, res_bitmap);

    bla = fft.execute_op2(&input);

    let mut res2 = PixelImageSimple { pixels: vec![], width: 0, height: 0 };

    let res2 = fft.get_output_bitmap();


    res2.save_png("fractal_fft_transformed.png");
}

fn sinus_wave_fft() {
    // create a sin wave
    let width = 256;
    let height = 256;
    let size = width * height;
    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];

    let mut sinus_bitmap = PixelImageSimple { pixels: bitmapdata.clone(), width: width, height: height };

    let mut val: f64;
    let mut a: f64 = 1.0 * 3.14159 / 180.0;
    let mut new_pixel: RGBA8 = RGBA8 {r:0, g: 0, b: 0, a: 0};

    for x in 0..width {
        val = a* (x as f64);
        let mut bla2: u8 = (val.sin()*255.0).abs() as u8;
        new_pixel.r = bla2;
        new_pixel.g = bla2;
        new_pixel.b = bla2;
        new_pixel.a = 255;

        for y in 0..height {
            sinus_bitmap.set_pixel(x, y, new_pixel);
        }
    }

    sinus_bitmap.save_png("sinus_fft_orig.png");

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
    let mut fft = ImageOperationFFT { input_bitmapdata: bitmap.clone(), output_bitmapdata: PixelImageSimple { pixels: bitmapdata, width: width, height: height } };

    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];

    fft.set_input_bitmap(sinus_bitmap);

    let res_bitmap: Vec<RGBA8> = vec![];
    let mut input = ImageOperationParam::new2(0, 0, res_bitmap.clone());
    let mut bla = ImageOperationParam::new2(0, 0, res_bitmap);

    bla = fft.execute_op2(&input);

    let mut res2 = PixelImageSimple { pixels: vec![], width: 0, height: 0 };

    let res2 = fft.get_output_bitmap();

    for y in 0..res2.height {
        for x in 0..res2.width {
            let pixel = res2.get_pixel(x, y);
            // println!("pixel  {}/{} = r: {}, g: {}, b: {}, a: {}", x, y, pixel.r, pixel.g,  pixel.b,  pixel.a);
        }
    }

    res2.save_png("sinus_fft_transformed.png");
}

fn main () {
    //sinus_wave_fft();
    //fractal();
    //white_square();
    // create_stripes();
    white_rectangle();
}
