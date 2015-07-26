trait ImageOp<'a> {
    type ImageData;

    fn execute_op(&self);
}

struct PixelImageSimple<'a> {
    pixels: &'a Vec<i32>,
    width: i32,
    height: i32,
}

struct ImageOperationSharpen<'a> {
    val: i32,
    bitmapdata: &'a PixelImageSimple<'a>
}

impl<'a> ImageOp<'a> for ImageOperationSharpen<'a> {
    type ImageData = PixelImageSimple<'a>;
    fn execute_op(&self) {
        println!("ImageOperationSharpen - val = {}, width = {}, height = {}, pixels = {:?}", &self.val, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}

//struct Image<'a> {
//    image_operations: Vec<Box<&'a ImageOp<ImageData=PixelImageSimple> + 'a>>
//    image_operations: Vec<Box<&'a (ImageOp<ImageData = PixelImageSimple> + 'a)>>
//}

//impl<'a> Image<'a> {
//  fn new() -> Image<'a> {
//       Image { image_operations: vec![] }
//   }

//    fn add_op(&mut self, image_ops: Box<ImageOp<ImageData=PixelImageSimple<'a> >>) {
//           self.image_operations.push(image_ops);
//    }
//}

fn main () {
    let bla = vec![1,2,3];

    let bitmap = PixelImageSimple { pixels: &bla, width: 222, height:334 };

    let sharpen = ImageOperationSharpen { val: 23, bitmapdata: &bitmap };
    let sharpen2 = ImageOperationSharpen { val: 34, bitmapdata: &bitmap };

    let obj = Box::new(sharpen) as Box<ImageOp<ImageData=PixelImageSimple>>;
    let obj2 = Box::new(sharpen2) as Box<ImageOp<ImageData=PixelImageSimple>>;

    obj.execute_op();
    obj2.execute_op();

//    let mut image = Image::new();

//    image.add_op(obj);
//    image.add_op(obj2);

    println!("execute_op()");
//    for imageops in image.image_operations.iter() {
//        imageops.execute_op();
//    }
}
