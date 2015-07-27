use std::rc::Rc;

use pixel_image_simple::PixelImageSimple;
use image_operation::ImageOperation;

pub struct ImageOperationRotate {
    pub angle: f64,
    pub bitmapdata: Rc<PixelImageSimple>
}

impl ImageOperation for ImageOperationRotate {
    fn execute_op(&self) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}, pixels = {:?}",
            &self.angle, &self.bitmapdata.width, &self.bitmapdata.height,&self.bitmapdata.pixels);
    }
}
