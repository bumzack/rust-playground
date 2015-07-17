struct PixelImageSimple {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

impl PixelImageSimple {
    fn new() -> PixelImageSimple {
        PixelImageSimple {
            pixels: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn set_width(&mut self, width: i32) ->  &mut PixelImageSimple {
        self.width  = width;
        self
    }

    fn get_width(&mut self) -> i32 {
        self.width
    }
}

struct ImageOperationReplaceColor<'a>  {
    color_old: f64,
    color_new: f64,
    bitmap: &'a PixelImageSimple,
    name: &'static str,
}

trait ImageOperation<'a>{
    fn set_bitmap(&mut self, bitmap: &'a PixelImageSimple);
    fn name(&self) -> &'static str;
}

impl<'a> ImageOperation<'a>  for ImageOperationReplaceColor  <'a> {
    // Replace `Self` with the implementor type: `ImageOperationReplaceColor`

    fn set_bitmap(&mut self, bitmap: &'a PixelImageSimple) {
        self.bitmap = bitmap;
    }

    // TODO: move this to the "ImageOperation" trait or struct ?
    fn name(&self) -> &'static str {
        self.name
    }
}

fn main () {
    let mut imagesimple = PixelImageSimple::new();
    let width: i32 = 100;

    imagesimple.set_width(width);

    //TODO: convert the "ImageOperationSharpenColor" constructor to the Builder Logic like  PixelImageSimple
    let sharpen_filter_op = Box::new(ImageOperationReplaceColor {
            name : "Sharpen Filter",
            color_old: 0.67,
            color_new: 0.89,
            bitmap: &imagesimple
    });
    let sharpen_filter_op2 = Box::new(ImageOperationReplaceColor {
            name : "Sharpen Filter2",
            color_old: 0.67,
            color_new: 0.89,
            bitmap: &imagesimple
    });
}
