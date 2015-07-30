use std::rc::Rc;

use std::f64;

use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;
use pixel_image_simple::RGBA8;

use image_operation_sharpen::ImageOperationSharpen;
use image_operation_rotate::ImageOperationRotate;

use image::Image;

use palette::PaletteRGBA8;
use palette_rainbow_colors::PaletteRainbowColorsRGBA8;

mod pixel_image_simple;
mod image_operation;
mod image_operation_rotate;
mod image_operation_sharpen;
mod palette;
mod palette_rainbow_colors;
mod image;

fn example_image() {
    let mut width = 2000;
    let mut height = 1000;
    let mut size = width * height;

     let mut bitmapdata: Vec<RGBA8> = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];

     for i in 0..size as usize {
        bitmapdata[i] = RGBA8 {r:0, g: 0, b: 0, a: 0};
     }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    let sharpen = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: bitmap };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_rotate);

    let mut bitmapdata: Vec<RGBA8> = vec![];

    let mut finished_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

    let bla = image.image_operations.len();

    for i in 0..bla as i32 {
        let mut input: Vec<ImageOperationParam> = vec![];
        let mut output: Vec<ImageOperationParam> = vec![];

        let mut bitmapdata: Vec<RGBA8> = vec![];

        let mut tmp_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

        let idx = i as usize;

        input = image.image_operations[idx].prepare_op();

        for param in &input  {
             let dummy: ImageOperationParam = image.image_operations[idx].execute_op2(param);
             output.push(dummy);
        }

        let tmp_bitmap = image.image_operations[idx].merge_results(output);

        if idx < image.image_operations.len()-1 {
            image.image_operations[idx + 1].set_input_bitmap(tmp_bitmap);
        } else {
            finished_bitmap = tmp_bitmap;
        }
    }
}

fn sinus() {
    // create a sin wave
    let width = 2000;
    let height = 1000;
    let size = width * height;
    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];

    let mut sinus_bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };

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

    sinus_bitmap.save_png("sinus_lode_big3.png");
}

fn fractal () {

    let width = 1600;
    let height = 1200;
    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];

    let palette = PaletteRainbowColorsRGBA8::new();

    for i in 0..palette.palette.len() {
        println!("palette[{}]  r: {}, g: {}, b: {}, a: {}", i,
            palette.palette[i].r, palette.palette[i].g, palette.palette[i].b, palette.palette[i].a)
    }

    let mut factal_bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };
    factal_bitmap.create_fractal(palette);
    let mut pixel = RGBA8 {r:0, g: 0, b: 0, a: 0};

    for x in 0..width {
        for y in 0..height {
            pixel = factal_bitmap.get_pixel(x, y);
            //println!("factal_bitmap.pixel   x: {}, y: {}         r: {}, g: {}, b: {}, a: {}",
             //  x, y, pixel.r, pixel.g, pixel.b, pixel.a);
        }
    }
    factal_bitmap.save_png("fractal2.png");
}

fn fractal2 () {
    let palette = PaletteRainbowColorsRGBA8::new();

    let width: i32 = (10 * palette.palette.len()) as i32;
    let height: i32 = 20 ;
    let bitmapdata = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; (width*height) as usize];
    let mut test = PixelImageSimple { pixels: bitmapdata, width: width, height: height };
    let mut pixel = RGBA8 {r:0, g: 0, b: 0, a: 0};
    let mut x: i32 = 0;

    for i in 0..palette.palette.len() {
        pixel = palette.palette[i];
        //println!("pixel  r: {}, g: {}, b: {}, a: {}",
        //    pixel.r, pixel.g, pixel.b, pixel.a);
        for j in 0..10 {
            x = (i as i32) * 10 + (j as i32);
            for y in 0..20 {
                test.set_pixel(x, y, pixel);
            }
        }
    }

    for x in 0..width {
        for y in 0..height {
            pixel = test.get_pixel(x, y);
            //println!("pixel x: {}, y: {}         r: {}, g: {}, b: {}, a: {}",
            //   x, y, pixel.r, pixel.g, pixel.b, pixel.a);
        }
    }
    test.save_png("test2.png");
}



fn main () {
    //sinus();
    fractal();
}
