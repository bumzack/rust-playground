struct PixelImageSimple<'a> {
    pixels: &'a Vec<i32>,
    width: i32,
    height: i32,
}

trait ImageOperation<'a> {
    fn execute_op(&self);
}

struct ImageOperationSharpen<'a> {
    val: i32,
    bitmapdata: &'a PixelImageSimple<'a>
}

impl<'a> ImageOperation<'a> for ImageOperationSharpen<'a> {
    fn execute_op(&self) {
        println!("ImageOperationSharpen - val = {}, width = {}, height = {}, pixels = {:?}",
            &self.val, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

struct ImageOperationRotate<'a> {
    angle: f64,
    bitmapdata: &'a PixelImageSimple<'a>
}

impl<'a> ImageOperation<'a> for ImageOperationRotate<'a> {
    fn execute_op(&self) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}, pixels = {:?}",
            &self.angle, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

struct Image<'a> {
    image_operations: Vec<Box<ImageOperation<'a>>>
}

impl<'a> Image<'a> {
    fn new() -> Image<'a> {
        Image { image_operations: vec![] }
    }

    fn add_op(&mut self, image_ops: Box<ImageOperation<'a>>) {
           self.image_operations.push(image_ops);
    }
}

fn main () {
    let bitmapdata = vec![1,2,3];

    let bitmap = PixelImageSimple { pixels: &bitmapdata, width: 222, height:334 };

    let sharpen = ImageOperationSharpen { val: 34, bitmapdata: &bitmap };
    let rotate = ImageOperationRotate { angle: 13.32, bitmapdata: &bitmap };

    let box_sharpen = Box::new(sharpen);
    let box_rotate = Box::new(rotate);

/*
    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_rotate);

    println!("execute_op()");
    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }
*/
}
