use std::rc::Rc;

use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;

// use image_operation_sharpen::ImageOperationSharpen;
use image_operation_rotate::ImageOperationRotate;

use image::Image;


mod pixel_image_simple;
mod image_operation;
mod image_operation_rotate;
// mod image_operation_sharpen;
mod image;


fn main () {
    let width: i32 = 100;
    let height: i32 = 50;
    let size = (width * height) as usize;
    let mut bitmapdata: Vec<i32> = Vec::with_capacity(size);

    let x = bitmapdata.len();
    for i in 0..x as usize {
        bitmapdata[i] = 1;
    }

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: width, height: height });

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

    for param  in &input  {
        println!("startx: {}, starty: {}, endx: {}, endy: {}", param.startx, param.starty, param.endx, param.endy);

        let dummy: ImageOperationParam = image.image_operations[0].execute_op2(param);
        output.push(dummy);
    }

    println!("");
    println!("");

    for param  in &output  {
         println!("OUTPUT     startx: {}, starty: {}, endx: {}, endy: {}", param.startx, param.starty, param.endx, param.endy);

    }
}
