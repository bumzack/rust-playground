use std::rc::Rc;
use std::path::Path;
use std::fs::File;

use std::f64;

extern crate image as png_image;



use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;

use image_operation_sharpen::ImageOperationSharpen;
use image_operation_rotate::ImageOperationRotate;

use image::Image;

mod pixel_image_simple;
mod image_operation;
mod image_operation_rotate;
mod image_operation_sharpen;
mod image;

fn main () {
    let mut width = 200;
    let mut height = 100;
    let mut size = width * height;

    let mut bitmapdata: Vec<i32> = vec![0; size as usize];

     for i in 0..size as usize {
        bitmapdata[i] = i as i32;
     }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    //println!("MAIN - INPUT pixels: {:?}", & bitmap.pixels);

    let sharpen = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: bitmap };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_rotate);

    let mut bitmapdata: Vec<i32> = vec![];
    let mut finished_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

    let bla = image.image_operations.len();
    println!("MAIN      bla = image.image_operations.len(): {:?}", bla);

    for i in 0..bla as i32 {
        let mut input: Vec<ImageOperationParam> = vec![];
        let mut output: Vec<ImageOperationParam> = vec![];

        let mut bitmapdata: Vec<i32> = vec![];
        let mut tmp_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

        let idx = i as usize;
        println!("LOOP      idx =  {:?}", idx);

        input = image.image_operations[idx].prepare_op();

        for param in &input  {
             let dummy: ImageOperationParam = image.image_operations[idx].execute_op2(param);
             output.push(dummy);
        }

        let tmp_bitmap = image.image_operations[idx].merge_results(output);
        println!("LOOP    idx: {},    tmp_bitmap.width: {:?}", idx, tmp_bitmap.width);
        println!("LOOP    idx: {},    tmp_bitmap.height: {:?}", idx, tmp_bitmap.height);
        println!("LOOP    idx: {},    tmp_bitmap: {:?}", idx, tmp_bitmap.pixels);

        if idx < image.image_operations.len()-1 {
            println!("LOOP    setting input bitmap  idx =  {:?}", idx);
            image.image_operations[idx + 1].set_input_bitmap(tmp_bitmap);
        } else {
            finished_bitmap = tmp_bitmap;
        }
    }
    println!("OUTPUT     finished_bitmap.pixels: {:?}", finished_bitmap.pixels);


    // create a sin wave
    width = 2000;
    height = 1000;
    size = width * height;
    bitmapdata = vec![0; size as usize];
    let mut bitmap = PixelImageSimple { pixels: bitmapdata, width: width, height: height };

    let mut val: f64;
    let mut a: f64 = 1.0 * 3.14159 / 180.0;
    // let mut a: f64 = 1.0;
    for x in 0..width {
        val = a* (x as f64);
        println!("SINUS x: {}, val: {}, val.sin(): {}", x, val, val.sin()*255.0);
        let mut bla2: i32 = (val.sin()*250.0).abs() as i32;
        for y in 0..height {
            bitmap.set_pixel(x, y, bla2);
        }
    }

    for x in 0..width {
        println!("SINUS      bitmap.get_pixel(): {:?}", bitmap.get_pixel(x, 0));
    }

    let mut imgbuf = png_image::ImageBuffer::new(width as u32, height  as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = bitmap.get_pixel(x as i32, y as i32);
        *pixel = png_image::Luma([i as u8]);
   }
   let ref mut fout = File::create(&Path::new("sinus.png")).unwrap();
   let _ = png_image::ImageLuma8(imgbuf).save(fout, png_image::PNG);
}

#[test]
fn test_rotate_sharpen_filter() {
    let width = 2000;
    let height = 1000;
    let size = width * height;

    let mut bitmapdata: Vec<i32> = vec![0; size as usize];

     for i in 0..size as usize {
        bitmapdata[i] = i as i32;
     }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    //println!("MAIN - INPUT pixels: {:?}", & bitmap.pixels);

    let sharpen = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: bitmap };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_rotate);

    let mut bitmapdata: Vec<i32> = vec![];
    let mut finished_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

    let bla = image.image_operations.len();
    println!("MAIN      bla = image.image_operations.len(): {:?}", bla);

    for i in 0..bla as i32 {
        let mut input: Vec<ImageOperationParam> = vec![];
        let mut output: Vec<ImageOperationParam> = vec![];

        let mut bitmapdata: Vec<i32> = vec![];
        let mut tmp_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

        let idx = i as usize;
        println!("LOOP      idx =  {:?}", idx);

        input = image.image_operations[idx].prepare_op();

        for param in &input  {
             let dummy: ImageOperationParam = image.image_operations[idx].execute_op2(param);
             output.push(dummy);
        }

        let tmp_bitmap = image.image_operations[idx].merge_results(output);
        println!("LOOP    idx: {},    tmp_bitmap.width: {:?}", idx, tmp_bitmap.width);
        println!("LOOP    idx: {},    tmp_bitmap.height: {:?}", idx, tmp_bitmap.height);
        println!("LOOP    idx: {},    tmp_bitmap: {:?}", idx, tmp_bitmap.pixels);

        if idx < image.image_operations.len()-1 {
            println!("LOOP    setting input bitmap  idx =  {:?}", idx);
            image.image_operations[idx + 1].set_input_bitmap(tmp_bitmap);
        } else {
            finished_bitmap = tmp_bitmap;
        }
    }
    println!("OUTPUT     finished_bitmap.pixels: {:?}", finished_bitmap.pixels);

    let mut expected_result: Vec<i32> = vec![0; size as usize];
    for i in 0..size as usize {
        expected_result[i] = ((i*2)+1) as i32;
    }
    assert_eq!(expected_result, finished_bitmap.pixels);

}


#[test]
fn test_sharpen_rotate_sharpen_filter() {
    let width = 2000;
    let height = 1000;
    let size = width * height;

    let mut bitmapdata: Vec<i32> = vec![0; size as usize];

     for i in 0..size as usize {
        bitmapdata[i] = i as i32;
     }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    //println!("MAIN - INPUT pixels: {:?}", & bitmap.pixels);

    let sharpen = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let sharpen2 = ImageOperationSharpen { val: 31, bitmapdata: bitmap.clone() };
    let rotate = ImageOperationRotate { angle: 1.32, bitmapdata: bitmap };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);
    let box_sharpen2 = Box::new(sharpen2);

    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_rotate);
    image.add_op(box_sharpen2);

    let mut bitmapdata: Vec<i32> = vec![];
    let mut finished_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

    let bla = image.image_operations.len();
    println!("MAIN      bla = image.image_operations.len(): {:?}", bla);

    for i in 0..bla as i32 {
        let mut input: Vec<ImageOperationParam> = vec![];
        let mut output: Vec<ImageOperationParam> = vec![];

        let mut bitmapdata: Vec<i32> = vec![];
        let mut tmp_bitmap = PixelImageSimple { pixels: bitmapdata, width: 0, height: 0 };

        let idx = i as usize;
        println!("LOOP      idx =  {:?}", idx);

        input = image.image_operations[idx].prepare_op();

        for param in &input  {
             let dummy: ImageOperationParam = image.image_operations[idx].execute_op2(param);
             output.push(dummy);
        }

        let tmp_bitmap = image.image_operations[idx].merge_results(output);
        println!("LOOP    idx: {},    tmp_bitmap.width: {:?}", idx, tmp_bitmap.width);
        println!("LOOP    idx: {},    tmp_bitmap.height: {:?}", idx, tmp_bitmap.height);
        println!("LOOP    idx: {},    tmp_bitmap: {:?}", idx, tmp_bitmap.pixels);

        if idx < image.image_operations.len()-1 {
            println!("LOOP    setting input bitmap  idx =  {:?}", idx);
            image.image_operations[idx + 1].set_input_bitmap(tmp_bitmap);
        } else {
            finished_bitmap = tmp_bitmap;
        }
    }
    println!("OUTPUT     finished_bitmap.pixels: {:?}", finished_bitmap.pixels);

    let mut expected_result: Vec<i32> = vec![0; size as usize];
    for i in 0..size as usize {
        expected_result[i] = (((i*2)+1)*2) as i32;
    }
    assert_eq!(expected_result, finished_bitmap.pixels);
}


#[test]
fn test_rotate_filter() {
    let width = 22;
    let height = 11;
    let size = width * height;

    let mut bitmapdata: Vec<i32> = vec![0; size as usize];

    for i in 0..size as usize {
        bitmapdata[i] = i as i32;
    }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: bitmap };
    let box_rotate = Box::new(rotate);
    let mut image = Image::new();
    image.add_op(box_rotate);
    println!("execute_op()");
    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }

    let mut input: Vec<ImageOperationParam> = vec![];
    let mut output: Vec<ImageOperationParam> = vec![];

    input = image.image_operations[0].prepare_op();

    for param in &input  {
        let dummy: ImageOperationParam = image.image_operations[0].execute_op2(param);
        output.push(dummy);
    }

    let resulting_bitmap = image.image_operations[0].merge_results(output);

    let mut expected_result: Vec<i32> = vec![0; size as usize];

    for i in 0..size as usize {
        expected_result[i] = (i+1) as i32;
    }
    assert_eq!(expected_result, resulting_bitmap.pixels);
}

#[test]
fn test_sharpen_filter() {
    let width = 22;
    let height = 11;
    let size = width * height;

    let mut bitmapdata: Vec<i32> = vec![0; size as usize];

    for i in 0..size as usize {
        bitmapdata[i] = i as i32;
    }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });
    let sharpen = ImageOperationSharpen { val: 3, bitmapdata: bitmap };
    let box_sharpen = Box::new(sharpen);
    let mut image = Image::new();
    image.add_op(box_sharpen);
    println!("execute_op()");
    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }

    let mut input: Vec<ImageOperationParam> = vec![];
    let mut output: Vec<ImageOperationParam> = vec![];

    input = image.image_operations[0].prepare_op();

    for param in &input  {
        let dummy: ImageOperationParam = image.image_operations[0].execute_op2(param);
        output.push(dummy);
    }

    let resulting_bitmap = image.image_operations[0].merge_results(output);
    let mut expected_result: Vec<i32> = vec![0; size as usize];

    for i in 0..size as usize {
        expected_result[i] = (i*2) as i32;
    }
    assert_eq!(expected_result, resulting_bitmap.pixels);
}
