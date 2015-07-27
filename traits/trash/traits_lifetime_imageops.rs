trait TraitBitmapData {
    fn new(x: f64, y: f64) -> Self;
    fn x(&self) -> &f64;
    fn y(&self) -> &f64;
    fn distance_to(&self, other: &Self) -> f64 {
        ((*other.x() - *self.x()).powi(2) + (*other.y() - *self.y()).powi(2)).sqrt()
    }
}

struct BitmapData {
    x: f64,
    y: f64
}

impl TraitBitmapData for BitmapData {
    fn new(x: f64, y: f64) -> Self {
         BitmapData { x: x, y: y }
    }
    fn x(&self) -> &f64 { &self.x }
    fn y(&self) -> &f64 { &self.y }
}

trait TraitBitmapDataOperation<'a> {
    type BitmapData: TraitBitmapData;

    fn new(topleft: &'a Self::BitmapData, bottomright: &'a Self::BitmapData) -> Self;
    fn topleft(&self) -> &Self::BitmapData;
    fn bottomright(&self) -> &Self::BitmapData;

    fn height(&self) -> f64 {
        (*self.topleft().y() - *self.bottomright().y()).abs()
    }

    fn width(&self) -> f64 {
        (*self.topleft().x() - *self.bottomright().x()).abs()
    }

    fn area(&self) -> f64 {
        (self.height() * self.width())
    }

    fn saymyname(&self) ;
}

struct ImageOpSharpen<'a> {
    tl: &'a BitmapData,
    br: &'a BitmapData
}

impl<'a> TraitBitmapDataOperation<'a> for ImageOpSharpen<'a> {
    type BitmapData = BitmapData;

    fn new(topleft: &'a BitmapData, bottomright: &'a BitmapData) -> ImageOpSharpen<'a> {
        ImageOpSharpen { tl: topleft, br: bottomright }
    }

    fn topleft(&self) -> &BitmapData {
        self.tl
    }

    fn bottomright(&self) -> &BitmapData {
        self.br
    }

    fn saymyname(&self) {
      println!("i am ImageOpSharpen");
    }
}

struct ImageOpRotate<'a> {
    tl: &'a BitmapData,
    br: &'a BitmapData
}

impl<'a> TraitBitmapDataOperation<'a> for ImageOpRotate<'a> {
    type BitmapData = BitmapData;

    fn new(topleft: &'a BitmapData, bottomright: &'a BitmapData) -> ImageOpRotate<'a> {
        ImageOpRotate { tl: topleft, br: bottomright }
    }

    fn topleft(&self) -> &BitmapData {
        self.tl
    }

    fn bottomright(&self) -> &BitmapData {
        self.br
    }

    fn saymyname(&self) {
      println!("i am ImageOpRotate");
    }
}



struct Image<'a> {
    image_operations: Vec<Box<TraitBitmapDataOperation<BitmapData=BitmapData> + 'a>>
    //    image_operations: Vec<Box<ImageOp<ImageData=PixelImageSimple> + 'a>>

}

impl<'a> Image<'a> {
   fn new() -> Image<'a> {
        Image { image_operations: vec![] }
    }

//    fn add_op(&mut self, image_ops: Box<TraitBitmapDataOperation>) {
//           self.image_operations.push(image_ops);
//    }
}

fn main() {
    let tl_p = BitmapData { x: 1.1, y: 2.2 };
    let br_p = BitmapData { x: 3.3, y: 4.4 };
    let sharpen = ImageOpSharpen { tl: &tl_p, br: &br_p };
    let rotate = ImageOpRotate { tl: &tl_p, br: &br_p };

    sharpen.saymyname();
    rotate.saymyname();

    println!("Width: {}", sharpen.width());
    println!("Height: {}", sharpen.height());
    println!("AREA: {}", sharpen.area());
}
