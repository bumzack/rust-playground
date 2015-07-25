#![feature(box_syntax)]

trait ImageOps {
    fn example_method(&self);
}

struct Image<'a> {
    things: Vec<Box<ImageOps + 'a>>
}

struct ExampleThing1 {
    just_some_random_data: i32
}

struct ExampleThing2 {
    other_random_data: f64
}

impl ImageOps for ExampleThing1 {
    fn example_method(&self) {
        println!("Example trait impl for ExampleThing 1");
    }
}
impl ImageOps for ExampleThing2 {
    fn example_method(&self) {
        println!("Example trait impl for ExampleThing 2");
    }
}
impl ExampleThing1 {
    fn new() -> ExampleThing1 {
        ExampleThing1 { just_some_random_data: 1}
    }
}

impl ExampleThing2 {
    fn new() -> ExampleThing2 {
        ExampleThing2 { other_random_data: 1.0}
    }
}

impl<'a> Image<'a> {
    fn new() -> Image<'a> {
        let bla = box ExampleThing1::new() as Box<ImageOps>;
        let bla2 = box ExampleThing2::new() as Box<ImageOps>;

        Image { things: vec![bla, bla2] }
    }
}

fn main() {
        let example = Image::new();
        for thing in example.things.iter() {
                thing.example_method();
        }
}
