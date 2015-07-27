// https://gist.github.com/amw-zero/55f5aa3a463154fb013d
// reddit discussion   http://www.reddit.com/r/rust/comments/2gl6av/new_to_rust_looking_for_help_with_something/

trait ExampleTrait {
    fn example_method(&self);
}

struct ExampleStruct<'a> {
    things: Vec<Box<ExampleTrait + 'a>>
}

struct ExampleThing1 {
    just_some_random_data: int
}

struct ExampleThing2 {
    other_random_data: f64
}

impl ExampleTrait for ExampleThing1 {
    fn example_method(&self) {
        println!("Example trait impl for ExampleThing 1");
    }
}
impl ExampleTrait for ExampleThing2 {
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

impl<'a> ExampleStruct<'a> {
    fn new() -> ExampleStruct<'a> {
        ExampleStruct { things: vec![box ExampleThing1::new() as Box<ExampleTrait>, box ExampleThing2::new() as Box<ExampleTrait>] }
    }
}

fn main() {
        let example = ExampleStruct::new();
        for thing in example.things.iter() {
                thing.example_method();
        }
}
