use std::rc::Rc;

use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;

use image_operation::ImageOperation;

pub struct ImageOperationRotate {
    pub angle: f64,
    pub bitmapdata: Rc<PixelImageSimple>
}

impl ImageOperation for ImageOperationRotate {
    fn execute_op(&self ) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}, pixels = {:?}",
            &self.angle, &self.bitmapdata.width, &self.bitmapdata.height, &self.bitmapdata.pixels);
    }

    fn execute_op2(&self, input: &ImageOperationParam) -> ImageOperationParam {
        let mut res = ImageOperationParam::new();
        res.start = &input.start+1;
        res.end = &input.end+1;
        res
    }

    fn prepare_op(&self) -> Vec<ImageOperationParam> {
        let mut res: Vec<ImageOperationParam> = vec![];
        let length = 10;
        for i in 0..length {
            let mut dummy = ImageOperationParam::new();
            dummy.start = i * length;
            dummy.end = dummy.start + length;
            res.push(dummy);
        }
        res
    }

    fn merge_results(&self, partial_results: Vec<ImageOperationParam>) -> PixelImageSimple {
        let bitmap = PixelImageSimple { pixels: vec![], width: 0, height:0 };
        bitmap
    }
}
