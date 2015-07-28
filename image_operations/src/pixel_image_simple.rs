pub struct PixelImageSimple {
    pub pixels: Vec<i32>,
    pub width: i32,
    pub height: i32,
}

pub struct ImageOperationParam {
    pub startx: i32,
    pub starty: i32,
    pub endx: i32,
    pub endy: i32,
    pub bitmap: PixelImageSimple
}

impl ImageOperationParam  {
    pub fn new() -> ImageOperationParam {
        let emtpy =  PixelImageSimple { pixels: vec![], width: 0, height:0 };
        ImageOperationParam { bitmap: emtpy, startx: 0, starty: 0, endx: 0, endy: 0 }
    }

    pub fn new2(width: i32, height: i32, bitmapdata: Vec<i32>) -> ImageOperationParam {
        let emtpy =  PixelImageSimple { pixels: bitmapdata, width: width, height: height };
        ImageOperationParam { bitmap: emtpy, startx: 0, starty: 0, endx: 0, endy: 0 }
    }
}
