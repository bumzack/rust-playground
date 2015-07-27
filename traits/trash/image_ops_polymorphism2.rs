struct PixelImageSimple<'a> {
    pixels: &'a Vec<i32>,
    width: i32,
    height: i32,
}

impl<'a> PixelImageSimple<'a>   {
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

impl<'a> ImageOps for ImageOpSharpen<'a> {
    fn example_method(&self) {
        println!("Example trait impl for ImageOpSharpen");
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



fn iterate_image_ops(all_ops: &[&ImageOps]) {
    for op in all_ops.iter() {
       op.example_method();
    }
}

fn main() {
    let imagesimple = PixelImageSimple::new();

    //  let monkey: &IndustrialRaverMonkey    = &Monster::new();
    let bla2: &ImageOpSharpen = &ImageOpSharpen::new2(&imagesimple);
    let bla3: &ImageOpSharpen = &ImageOpSharpen::new2(&imagesimple);

    //let all_ops: &[&ImageOps] = [bla2 as &ImageOps, bla3 as &ImageOps];

    //iterate_image_ops(all_ops);
}
