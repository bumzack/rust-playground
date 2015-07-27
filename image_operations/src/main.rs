use std::rc::Rc;

use pixel_image_simple::PixelImageSimple;
use image_operation_sharpen::ImageOperationSharpen;
use image_operation_rotate::ImageOperationRotate;
use image::Image;

mod pixel_image_simple;
mod image_operation;
mod image_operation_rotate;
mod image_operation_sharpen;
mod image;


fn main () {
    let bitmapdata = vec![1,2,3];

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: 222, height:334 });

    let sharpen = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: bitmap };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_rotate);

    println!("execute_op()");
    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }
}
