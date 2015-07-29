use std::rc::Rc;

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
    let width = 20;
    let height = 10;
    let size = width * height;

    let mut bitmapdata: Vec<i32> = vec![0; size as usize];

     for i in 0..size as usize {
        bitmapdata[i] = i as i32;
     }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

    //println!("MAIN - INPUT pixels: {:?}", & bitmap.pixels);

    //let sharpen = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: bitmap };

    //let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

    let mut image = Image::new();

    //image.add_op(box_sharpen);
    image.add_op(box_rotate);

    println!("execute_op()");
    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }

    let mut input: Vec<ImageOperationParam> = vec![];
    let mut output: Vec<ImageOperationParam> = vec![];

    input = image.image_operations[0].prepare_op();

    for param in &input  {
        println!("startx: {}, starty: {}, endx: {}, endy: {}", param.startx, param.starty, param.endx, param.endy);
        println!("after println");

        let dummy: ImageOperationParam = image.image_operations[0].execute_op2(param);
        //println!("after let dummy: ImageOperationParam");
        output.push(dummy);
    }

    println!("");
    println!("");

    for param  in &output  {
        println!("OUTPUT     startx: {}, starty: {}, endx: {}, endy: {}", param.startx, param.starty, param.endx, param.endy);
        //println!("OUTPUT     pixels: {:?}", param.bitmap.pixels);
    }

    let resulting_bitmap = image.image_operations[0].merge_results(output);
    println!("OUTPUT     resulting_bitmap.pixels: {:?}", resulting_bitmap.pixels);
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
