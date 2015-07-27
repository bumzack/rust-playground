#![feature(box_syntax)]

struct PixelImageSimple {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

trait ImageOperation<'a> {
    fn show_name(&self);
    fn set_bitmap(&mut self, bmp: &PixelImageSimple);
    fn new(bitmap : &'a PixelImageSimple) -> Self;
}

struct ImageOperationReplaceColor<'a>   {
    value: i32,
    bitmap: &'a PixelImageSimple
}

impl<'a> ImageOperation<'a> for ImageOperationReplaceColor<'a> {
    fn new(bmp : &'a PixelImageSimple ) -> ImageOperationReplaceColor<'a> {
        ImageOperationReplaceColor { value: 2, bitmap : bmp }
    }

    fn show_name(&self) {
        println!("i am 'ImageOperationReplaceColor.show_name'");
    }

    fn set_bitmap(&mut self, bmp: &PixelImageSimple) {
        println!("i am 'ImageOperationReplaceColor.set_bitmap'");
    }
}


// http://www.reddit.com/r/rust/comments/2gl6av/new_to_rust_looking_for_help_with_something/
// https://gist.github.com/amw-zero/55f5aa3a463154fb013d
struct Image<'a> {
    image_operations: Vec<Box<ImageOperation<'a> + 'a>>
}

impl<'a> Image<'a>   {
    // add an image operation to the array of image operation
    fn new() -> Image<'a> {
        Image { image_operations: vec![] }
    }
}

fn main () {
    let imagesimple = PixelImageSimple {
            pixels: vec!(),
            width: 0,
            height: 0,
    };

    let filter1: ImageOperationReplaceColor = ImageOperationReplaceColor {
        value: 2,
        bitmap: &imagesimple
    };


    let filter2: ImageOperationReplaceColor = ImageOperationReplaceColor {
        value: 3,
        bitmap: &imagesimple
    };

    let image = Image::new();
    for op in image.image_operations.iter() {
    //    op.show_name();
    }

    // for op in image.image_operations.iter() {
    //     op.set_bitmap(&imagesimple);
    //}
}
