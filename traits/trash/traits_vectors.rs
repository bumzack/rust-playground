// http://stackoverflow.com/questions/27675554/type-definition-with-a-trait-differences-of-specifying-an-explicit-lifetime-bou


trait Kind {
    fn trait_fn(&self) -> u8 { 0 }
}

type CollectionOfKind1<'a> = Vec<&'a (Kind + 'static)>;
type CollectionOfKind2<'a> = Vec<&'a (Kind + 'a)>;

struct Alpha;
impl Kind for Alpha {}

struct Beta<'b> {
    name: &'b str,
}
impl<'a> Kind for Beta<'a> {}

fn main() {
    let name = "world".to_string();

    // Doesn't need/have it's own lifetime.
    let a = Alpha;
    // Has a reference to something with the 'static lifetime.
    let b1 = Beta { name: "hello" };
    // Has a reference to something with the lifetime of `name`,
    // which is less than 'static.
    let b2 = Beta { name: &name[..] };

    // Our vector is composed of references to
    // things that *might* have a reference themselves!
    let mut c1: CollectionOfKind1 = Vec::new();
    c1.push(&a);
    c1.push(&b1);
    c1.push(&b2); // error: `name` does not live long enough

    let mut c2: CollectionOfKind2 = Vec::new();
    c2.push(&a);
    c2.push(&b1);
    c2.push(&b2); // Hooray
}
