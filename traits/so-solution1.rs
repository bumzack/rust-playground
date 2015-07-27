use std::rc::Rc;

struct PixelImageSimple {
    pixels: Vec<i32>,
    width: i32,
    height: i32,
}

trait ImageOperation {
    fn execute_op(&self);
}

struct ImageOperationSharpen {
    val: i32,
    bitmapdata: Rc<PixelImageSimple>
}

impl ImageOperation for ImageOperationSharpen {
    fn execute_op(&self) {
        println!("ImageOperationSharpen - val = {}, width = {}, height = {}, pixels = {:?}",
            &self.val, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

struct ImageOperationRotate {
    angle: f64,
    bitmapdata: Rc<PixelImageSimple>
}

impl ImageOperation for ImageOperationRotate {
    fn execute_op(&self) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}, pixels = {:?}",
            &self.angle, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

struct Image {
    image_operations: Vec<Box<ImageOperation>>
}

impl Image {
    fn new() -> Image {
        Image { image_operations: vec![] }
    }

    fn add_op(&mut self, image_ops: Box<ImageOperation>) {
           self.image_operations.push(image_ops);
    }
}

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
