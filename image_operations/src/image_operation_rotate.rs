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
        res.startx = &input.startx + 1;
        res.starty = &input.starty + 1;
        res.endx = &input.endx + 1;
        res.endy = &input.endy + 1;

        // this is gonna be THE algorithm sometimes
        // here just add 1 to the input values
        res
    }

    fn prepare_op(&self) -> Vec<ImageOperationParam> {
        let mut res: Vec<ImageOperationParam> = vec![];
        // This is decided by the algorithm - how many parts to divide the bitmap into
        let count_parts = 3;
        let part_width = &self.bitmapdata.width / (count_parts );
        let part_height = &self.bitmapdata.height / (count_parts);

        for x in 0..(count_parts-1) {
            for y in 0..(count_parts-1) {
                let mut dummy = ImageOperationParam::new();
                dummy.startx = x * part_width;
                dummy.starty = y * part_height;
                dummy.endx = dummy.startx + part_width;
                dummy.endy = dummy.starty + part_height;
                res.push(dummy);
            }
        }
        // to make sure the whole image is covered
        let mut dummy = ImageOperationParam::new();
        dummy.startx = (count_parts-1) * part_width;
        dummy.starty = (count_parts-1) * part_height;
        dummy.endx = self.bitmapdata.width;
        dummy.endy = self.bitmapdata.height;
        res.push(dummy);
        res
    }

    fn merge_results(&self, partial_results: Vec<ImageOperationParam>) -> PixelImageSimple {
        let bitmap = PixelImageSimple { pixels: vec![], width: 0, height:0 };
        bitmap
    }
}
