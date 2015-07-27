// https://www.reddit.com/r/rust/comments/30ehed/why_must_this_reference_have_a_static_lifetime/

struct Heaven;

trait Celestial {
    fn dummy(&self) -> &'static str { "Dummy" }
}

struct Sky<'a> {
    v: Vec<Box<Celestial + 'a>>,
    h: &'a Heaven,
}

struct Venus<'a> {
    h: &'a Heaven
}

impl<'a> Celestial for Venus<'a> {}

fn main() {
    let h = Heaven;
    {
        let mut s = Sky { v: vec!(), h: &h };
        s.v.push(Box::new(Venus { h: &h }));
    }
}
