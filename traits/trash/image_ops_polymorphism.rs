struct PixelImageSimple<'a> {
    pixels: &'a Vec<i32>,
    width: i32,
    height: i32,
}

impl<'a>  PixelImageSimple<'a>   {
    fn new() -> PixelImageSimple<'a>  {
        PixelImageSimple {
            pixels: &vec![],
            width: 10,
            height: 10,
        }
    }
}

trait ImageOps {
    fn example_method(&self);
}

struct ImageOpSharpen<'a> {
    just_some_random_data: i32,
    img: &'a PixelImageSimple<'a>
}

struct ImageOpReplaceColor {
    other_random_data: f64
}

impl<'a> ImageOps for ImageOpSharpen<'a> {
    fn example_method(&self) {
        println!("Example trait impl for ImageOpSharpen");
    }
}
impl ImageOps for ImageOpReplaceColor {
    fn example_method(&self) {
        println!("Example trait impl for ImageOpReplaceColor");
    }
}

impl<'a> ImageOpSharpen<'a> {
    //fn new() -> ImageOpSharpen<'a> {
    //    ImageOpSharpen   { just_some_random_data: 1, img:PixelImageSimple::new() }
    //}
    fn new2(image: &'a PixelImageSimple<'a>) -> ImageOpSharpen<'a>  {
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
        Image { image_operations: vec![] }
    }

    fn add_op(&mut self, image_ops: Box<ImageOps>) {
           self.image_operations.push(image_ops);
    }
}

fn main() {
    let mut image = Image::new();
    let imagesimple = PixelImageSimple::new();
    let bla2 = Box::new(ImageOpSharpen::new2(&imagesimple)) as Box<ImageOps>;
    let bla3 = Box::new(ImageOpSharpen::new2(&imagesimple)) as Box<ImageOps>;
    let bla4 = Box::new(ImageOpSharpen::new2(&imagesimple)) as Box<ImageOps>;

//    image.add_op(bla2);
//    image.add_op(bla3);
//    image.add_op(bla4);

    for imageop in image.image_operations.iter() {
//        imageop.example_method();
    }
}
