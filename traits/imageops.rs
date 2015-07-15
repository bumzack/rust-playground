struct PixelImage {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

impl PixelImage {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn new() -> PixelImage {
        println!("PixelImage::new()");
        PixelImage { pixels: Vec::new(), width: 0, height: 0 }
    }
}

struct ImageOperationResult  {
    subimage: PixelImage,
    width: i32,
    height: i32,
    startx: i32,
    starty: i32,
}

trait ImageOperation {
    fn executeOp(&self) -> ImageOperationResult;
}


struct ImageOp {
    Image: PixelImage,
}

fn main () {
    let image = PixelImage {
        // Static methods are called using double colons
        width: 0,
        height: 0,
        pixels: Vec::new()
    };
}
