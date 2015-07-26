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
  val: i32
}



impl ImageOp for ImageOperationSharpen {
    type ImageData = PixelImageSimple;

    fn saymyname(&self, image_data: &PixelImageSimple) {
        println!("ImageOperationSharpen");
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

fn main () {
    let sharpen = ImageOperationSharpen { val: 23 };
    let rotate = ImageOperationRotate { degree: 23 };

     let obj = Box::new(sharpen) as Box<ImageOp<ImageData=PixelImageSimple>>;
    let obj2 = Box::new(rotate) as Box<ImageOp<ImageData=PixelImageSimple>>;

    obj.execute_op();
    obj2.execute_op();

}
