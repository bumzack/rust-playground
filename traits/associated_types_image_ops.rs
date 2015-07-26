trait ImageOp {
    type ImageData;

    fn saymyname(&self, &Self::ImageData);
    fn execute_op(&self);
}

struct PixelImageSimple {
    pixels: Vec<i32>,
    width: i32,
    height: i32,
}

struct ImageOperationSharpen {
    val: i32,
    bitmapdata: PixelImageSimple
}


impl ImageOp for ImageOperationSharpen {
    type ImageData = PixelImageSimple;

    fn saymyname(&self, image_data: &PixelImageSimple) {
    //    println!("ImageOperationSharpen - image_data.width {}", image_data.width);
    }

    fn execute_op(&self) {
        println!("ImageOperationSharpen");
    }
}




struct ImageOperationRotate {
  degree: i32
}


impl ImageOp for ImageOperationRotate {
    type ImageData = PixelImageSimple;

    fn saymyname(&self, image_data: &PixelImageSimple) {
        println!("ImageOperationRotate");
    }

    fn execute_op(&self) {
        println!("ImageOperationRotate");
    }
}



struct Image<'a> {
    image_operations: Vec<Box<ImageOp<ImageData=PixelImageSimple> + 'a>>
}

impl<'a> Image<'a> {
  fn new() -> Image<'a> {
       Image { image_operations: vec![] }
   }

    fn add_op(&mut self, image_ops: Box<ImageOp<ImageData=PixelImageSimple>>) {
           self.image_operations.push(image_ops);
    }
}

fn main () {

    let bitmap = PixelImageSimple { pixels: vec![1,2,3], width: 3, height:4 };
    let bitmap2 = PixelImageSimple { pixels: vec![4,5,6], width: 13, height: 14 };

    let sharpen = ImageOperationSharpen { val: 23, bitmapdata: bitmap };
    let rotate = ImageOperationRotate { degree: 23 };

     let obj = Box::new(sharpen) as Box<ImageOp<ImageData=PixelImageSimple>>;
    let obj2 = Box::new(rotate) as Box<ImageOp<ImageData=PixelImageSimple>>;

    obj.execute_op();
    obj2.execute_op();


    let mut image = Image::new();

    image.add_op(obj);
    image.add_op(obj2);

    for imageops in image.image_operations.iter() {
        imageops.execute_op();
    }
}
