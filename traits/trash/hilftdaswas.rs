extern crate std;

pub struct Graph<'a>
{
    vertices: Vec<Vertex>,
    edges: Vec<Edge<'a>>,
}

impl<'a> Graph<'a>
{
    pub fn new() -> Graph<'a>
    {
        let vertices = vec![];
        let edges = vec![];

        let inst = Graph {
            vertices: vertices,
            edges: edges
        };

        return inst;
    }
}

pub struct Vertex
{
    name: String
}

impl Vertex
{
    pub fn new(name: &str) -> Vertex
    {
        let inst = Vertex {
            name: name.to_string()
        };
        return inst;
    }
}

pub struct Edge<'a>
{
    from: &'a Vertex,
    to: &'a Vertex,
    latency: u32,
    throughput: u32,
}

impl<'a> Edge<'a>
{
    pub fn new(from: &'a Vertex, to: &'a Vertex) -> Edge<'a>
    {
        let inst = Edge {
            from: from,
            to: to,
            latency: 0,
            throughput: 0
        };
        return inst;
    }
}
