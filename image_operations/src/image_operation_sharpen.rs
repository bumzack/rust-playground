use std::rc::Rc;

use pixel_image_simple::PixelImageSimple;
use image_operation::ImageOperation;

pub struct ImageOperationSharpen {
    pub val: i32,
    pub bitmapdata: Rc<PixelImageSimple>
}

impl ImageOperation for ImageOperationSharpen {
    fn execute_op(&self) {
        println!("ImageOperationSharpen - val = {}, width = {}, height = {}, pixels = {:?}",
            &self.val, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}
