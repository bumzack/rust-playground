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

type CollectionOfImageOps<'a> = Vec<&'a (ImageOperation<'a> + 'a)>;

fn main () {
    let bitmapdata = vec![1,2,3];
    let bitmapdata2 = vec![1,2,3];

    let bitmap = PixelImageSimple { pixels: &bitmapdata, width: 222, height:334 };
    let bitmap2 = PixelImageSimple { pixels: &bitmapdata2, width: 111, height:289 };

    let dummy    = ImageOperationSharpen { val: 34, bitmapdata: &bitmap2 };
    let sharpen     = ImageOperationSharpen { val: 23, bitmapdata: &bitmap };

    let mut c2: CollectionOfImageOps = Vec::new();
    c2.push(&dummy);
    c2.push(&sharpen);

/*
    let mut image = Image::new();

    image.add_op(box_sharpen);
    image.add_op(box_sharpen2);

    println!("execute_op()");
    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }
*/
}
