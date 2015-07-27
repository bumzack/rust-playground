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
    bitmapdata: Rc<PixelImageSimple>,
}

impl ImageOperation for ImageOperationSharpen {
    fn execute_op(&self) {
        println!("ImageOperationSharpen - val = {}, width = {}, height = {}, pixels = {:?}",
            &self.val, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

struct ImageOperationRotate {
    angle: f64,
    bitmapdata: Rc<PixelImageSimple>,
}

impl ImageOperation for ImageOperationRotate {
    fn execute_op(&self) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}, pixels = {:?}",
            &self.angle, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

type CollectionOfImageOperations<'a> = Vec<&'a ImageOperation>;

fn main () {
    let bitmapdata = vec![1,2,3];

    let bitmap = Rc::new(PixelImageSimple { pixels: bitmapdata, width: 222, height:334 });

    let dummy    = ImageOperationSharpen { val: 34, bitmapdata: bitmap.clone() };
    let sharpen  = ImageOperationRotate { angle: 0.56, bitmapdata: bitmap };

    let mut image_operations: CollectionOfImageOperations = Vec::new();
    image_operations.push(&dummy);
    image_operations.push(&sharpen);
    println!("execute_op()");
    for imageops in image_operations.iter() {
        imageops.execute_op();
    }
}
