mod graph {
use std::rc::Rc;

pub struct Graph
{
    vertices: Vec<Vertex>,
    edges: Vec<Rc<Edge>>,
}

impl Graph
{
    pub fn new() -> Graph
    {
        Graph {
            vertices: vec![],
            edges: vec![],
        }
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
        Vertex {
            name: name.to_string()
        }
    }
}

pub struct Edge
{
    from: Rc<Vertex>,
    to: Rc<Vertex>,
    latency: u32,
    throughput: u32,
}

impl Edge
{
    pub fn new(from: Rc<Vertex>, to: Rc<Vertex>) -> Edge
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
}
