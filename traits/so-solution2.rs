// thx to http://stackoverflow.com/questions/31640442/how-can-i-use-lifetime-bounds-to-solve-reference-must-be-valid-for-the-static-l

#![feature(drain)]

struct PixelImageSimple {
    pixels: Vec<i32>,
    width: i32,
    height: i32,
}

trait ImageOperation {
    fn execute_op(&self, bitmap: &mut PixelImageSimple);
}

struct ImageOperationSharpen {
    val: i32,
 }

impl ImageOperation for ImageOperationSharpen {
    fn execute_op(&self, bitmap: &mut PixelImageSimple) {
        println!("ImageOperationSharpen - val = {}, width = {}, height = {}, pixels = {:?}",
            &self.val, bitmap.width, bitmap.height,bitmap.pixels);
    }
}

struct ImageOperationRotate {
    angle: f64,
 }

impl ImageOperation for ImageOperationRotate {
    fn execute_op(&self, bitmap: &mut PixelImageSimple) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}, pixels = {:?}",
            &self.angle, bitmap.width, bitmap.height, bitmap.pixels);
    }
}

struct Image {
    image_operations: Vec<Box<ImageOperation>>,
    bitmap: PixelImageSimple
}

impl Image {
    fn new(bitmap: PixelImageSimple) -> Image {
        Image {
            image_operations: vec![],
            bitmap: bitmap
        }
    }

    fn add_op(&mut self, image_ops: Box<ImageOperation>) {
           self.image_operations.push(image_ops);
    }


    fn apply_ops(&mut self) {
        // iterate over the ops and automatically remove them
        for op in self.image_operations.drain(..) {
            op.execute_op(&mut self.bitmap);
        }
    }
}

fn main () {
    let bitmapdata = vec![1,2,3];

    let bitmap =  PixelImageSimple { pixels: bitmapdata, width: 222, height:334 };

    let sharpen = ImageOperationSharpen { val: 34 };
    let rotate = ImageOperationRotate { angle: 13.32 };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

    let mut image = Image::new(bitmap);

    image.add_op(box_sharpen);
    image.add_op(box_rotate);

    println!("execute_op()");
    image.apply_ops();
}
