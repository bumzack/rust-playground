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

type CollectionOfImageOperations<'a> = Vec<&'a (ImageOperation<'a> + 'a)>;

fn main () {
    let bitmapdata = vec![1,2,3];

    let bitmap = PixelImageSimple { pixels: &bitmapdata, width: 222, height:334 };

    let dummy    = ImageOperationSharpen { val: 34, bitmapdata: &bitmap };
    let sharpen  = ImageOperationRotate { angle: 0.56, bitmapdata: &bitmap };

    let mut image_operations: CollectionOfImageOperations = Vec::new();
    image_operations.push(&dummy);
    image_operations.push(&sharpen);
}
