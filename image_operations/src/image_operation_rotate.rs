use std::rc::Rc;

use pixel_image_simple::PixelImageSimple;
use pixel_image_simple::ImageOperationParam;
use pixel_image_simple::RGBA8;


use image_operation::ImageOperation;

pub struct ImageOperationRotate {
    pub angle: f64,
    pub bitmapdata: Rc<PixelImageSimple>
}

impl ImageOperation for ImageOperationRotate {
    fn execute_op(&self ) {
        println!("ImageOperationRotate - angle = {}, width = {}, height = {}",
            &self.angle, &self.bitmapdata.width, &self.bitmapdata.height );
    }

    fn execute_op2(&self, input: &ImageOperationParam) -> ImageOperationParam {
        let width: i32 = &input.endx - &input.startx;
        let height: i32 = &input.endy - &input.starty;
        let size = (width * height) as usize;

        let res_bitmap: Vec<RGBA8> = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
        let mut res = ImageOperationParam::new2(width, height, res_bitmap);

        // this is gonna be THE algorithm sometimes
        // here just mltiply by  2
        let mut new_pixel: RGBA8 = RGBA8 {r:0, g: 0, b: 0, a: 0};
        let mut old_pixel: RGBA8 = RGBA8 {r:0, g: 0, b: 0, a: 0};

        for x in 0..width {
            for y in 0..height {
                let x2 = x + input.startx;
                let y2 = y + input.starty;
                old_pixel = self.bitmapdata.get_pixel(x2, y2);
                new_pixel.r = old_pixel.r + 1;
                new_pixel.g = old_pixel.g + 1;
                new_pixel.b = old_pixel.b + 1;
                new_pixel.a = old_pixel.a + 1;
                res.bitmap.set_pixel(x, y, new_pixel);
            }
        }

        res.startx = input.startx;
        res.starty = input.starty;
        res.endx = input.endx;
        res.endy = input.endy;

        // this is gonna be THE algorithm sometimes
        // here just add 1 to the input value

        res
    }

    // TODO: move to trait "ImageOperation" - they can be used indepent of the actual image operation
    fn prepare_op(&self) -> Vec<ImageOperationParam> {
        let mut res: Vec<ImageOperationParam> = vec![];
        // This is decided by the algorithm - how many parts to divide the bitmap into
        let count_parts = 8;
        let part_width = &self.bitmapdata.width / count_parts;
        let part_height = &self.bitmapdata.height / count_parts;

        for y in 0..count_parts {
            for x in 0..count_parts {
                let mut dummy = ImageOperationParam::new();
                dummy.startx = x * part_width;
                dummy.starty = y * part_height;
                dummy.endx = dummy.startx + part_width;
                dummy.endy = dummy.starty + part_height;
                if x == count_parts-1 {
                    dummy.endx = self.bitmapdata.width;
                }
                if y == count_parts-1 {
                    dummy.endy = self.bitmapdata.height;
                }
                res.push(dummy);
            }
        }

        // to make sure the whole image is covered
        let mut dummy = ImageOperationParam::new();
        dummy.startx = (count_parts-1) * part_width;
        dummy.starty = (count_parts-1) * part_height;
        dummy.endx = self.bitmapdata.width;
        dummy.endy = self.bitmapdata.height;
        //res.push(dummy);

        res
    }

    // TODO: move to trait "ImageOperation"
    fn merge_results(&self, partial_results: Vec<ImageOperationParam>) -> PixelImageSimple {
        let size = (self.bitmapdata.width * self.bitmapdata.height) as usize;
        let res_bitmap: Vec<RGBA8> = vec![RGBA8 {r:0, g: 0, b: 0, a: 0}; size as usize];
        let mut bitmap = PixelImageSimple { pixels: res_bitmap, width: self.bitmapdata.width, height: self.bitmapdata.height };

        for part in partial_results {
            let width  = part.bitmap.width;
            let height  = part.bitmap.height;

            for x in 0..width {
                for y in 0..height {
                    let x2 = x + part.startx;
                    let y2 = y + part.starty;

                    bitmap.set_pixel(x2, y2, part.bitmap.get_pixel(x, y));
                }
            }
        }
        bitmap
    }

    fn set_input_bitmap(&mut self, input_bitmap: PixelImageSimple) {
        self.bitmapdata = Rc::new(input_bitmap);
    }

    fn get_output_bitmap(&self) -> PixelImageSimple {
        PixelImageSimple { pixels: vec![], width: 0, height: 0 }
    }
}
