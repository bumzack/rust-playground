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
        let width: i32 = &input.endx - &input.startx;
        let height: i32 = &input.endy - &input.starty;
        let size = (width * height) as usize;

        let mut res_bitmap: Vec<i32> = vec![0; size as usize];

        let offset = input.startx + input.starty * self.bitmapdata.width;

        for x in 0..width {
            for y in 0..height {
                let idx = (y * width + x) as usize;
                let idx2 = (offset + y * width + x) as usize;
                res_bitmap[idx] = &self.bitmapdata.pixels[idx2] + 1;
            }
        }

        let mut res = ImageOperationParam::new2(width, height, res_bitmap);
        println!("execute_op2 - after 'let mut res'" );

        res.startx = input.startx;
        res.starty = input.starty;
        res.endx = input.endx;
        res.endy = input.endy;

        // this is gonna be THE algorithm sometimes
        // here just add 1 to the input value

        res
    }

    fn prepare_op(&self) -> Vec<ImageOperationParam> {
        let mut res: Vec<ImageOperationParam> = vec![];
        // This is decided by the algorithm - how many parts to divide the bitmap into
        let count_parts = 2;
        let part_width = &self.bitmapdata.width / count_parts;
        let part_height = &self.bitmapdata.height / count_parts;

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
        let size = (self.bitmapdata.width * self.bitmapdata.height) as usize;
        let mut res_bitmap: Vec<i32> = vec![0; size as usize];

        for part in partial_results {
            let offset = part.startx + part.starty * self.bitmapdata.width;
            let width  = part.bitmap.width;
            let height  = part.bitmap.height;

            for x in 0..width {
                for y in 0..height {
                    let idx = (y * width + x) as usize;
                    let idx2 = (offset + y * width + x) as usize;
                    res_bitmap[idx2] = part.bitmap.pixels[idx];
                }
            }
        }
        let bitmap = PixelImageSimple { pixels: res_bitmap, width: self.bitmapdata.width, height: self.bitmapdata.height };
        bitmap
    }
}
