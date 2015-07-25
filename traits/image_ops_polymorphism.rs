#![feature(box_syntax)]

trait ImageOps {
    fn example_method(&self);
}

struct ImageOpSharpen {
    just_some_random_data: i32
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
        ImageOpSharpen   { just_some_random_data: 1}
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
        let bla = box ImageOpSharpen::new() as Box<ImageOps>;
        let bla2 = box ImageOpReplaceColor::new() as Box<ImageOps>;

        Image { image_operations: vec![bla, bla2] }
    }
}

fn main() {
        let example = Image::new();
        for thing in example.image_operations.iter() {
                thing.example_method();
        }
}
