#![feature(box_syntax)]

struct PixelImageSimple {
    pixels:  Vec<i32>,
    width: i32,
    height: i32,
}

impl PixelImageSimple  {
    fn new() -> PixelImageSimple {
        PixelImageSimple {
            pixels: vec![],
            width: 10,
            height: 10,
        }
    }
}

trait ImageOps {
    fn example_method(&self);
}

struct ImageOpSharpen {
    just_some_random_data: i32,
    img: PixelImageSimple
}

struct ImageOpReplaceColor {
    other_random_data: f64
}

impl ImageOps for ImageOpSharpen {
    fn example_method(&self) {
        println!("Example trait impl for ImageOpSharpen");
    }
}
impl ImageOps for ImageOpReplaceColor {
    fn example_method(&self) {
        println!("Example trait impl for ImageOpReplaceColor");
    }
}
impl ImageOpSharpen {
    fn new() -> ImageOpSharpen {
        ImageOpSharpen   { just_some_random_data: 1, img:PixelImageSimple::new() }
    }
    fn new2(image: PixelImageSimple) -> ImageOpSharpen {
        ImageOpSharpen   { just_some_random_data: 1, img: image}
    }
}

impl ImageOpReplaceColor {
    fn new() -> ImageOpReplaceColor {
        ImageOpReplaceColor { other_random_data: 1.0}
    }
}

struct Image<'a> {
    image_operations: Vec<Box<ImageOps + 'a>>
}

impl<'a> Image<'a> {
    fn new() -> Image<'a> {
        let imagesimple = PixelImageSimple::new();
        let bla = box ImageOpSharpen::new() as Box<ImageOps>;
        let bla2 = box ImageOpReplaceColor::new() as Box<ImageOps>;
        let bla3 = box ImageOpSharpen::new2(imagesimple) as Box<ImageOps>;

        Image { image_operations: vec![bla, bla2, bla3] }
    }

    fn add_op(&mut self, image_ops: Box<ImageOps>) {
           self.image_operations.push(image_ops);
    }
}

fn main() {
    let imagesimple = PixelImageSimple::new();
        let example = Image::new();
        for thing in example.image_operations.iter() {
                thing.example_method();
        }
}
