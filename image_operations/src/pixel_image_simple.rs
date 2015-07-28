use std::rc::Rc;

pub struct PixelImageSimple {
    pub pixels: Vec<i32>,
    pub width: i32,
    pub height: i32,
}

pub struct ImageOperationParam {
    pub start: i32,
    pub end: i32,
    pub bitmap: PixelImageSimple
}

impl ImageOperationParam  {
    pub fn new() -> ImageOperationParam {
        let mut bitmapdata = vec![];
        let emtpy =  PixelImageSimple { pixels: bitmapdata, width: 0, height:0 };
        ImageOperationParam { bitmap: emtpy, start: 0, end: 0 }
    }
}
