pub struct Graph<'a,'b>
{
    vertices: Vec<Vertex>,
    edges: Vec<Edge<'a,'b>>,
}

impl<'a,'b> Graph<'a,'b>
{
    pub fn new() -> Graph<'a,'b>
    {
        let vertices = vec![];
        let edges = vec![];

        Graph {
            vertices: vertices,
            edges: edges
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

pub struct Edge<'a,'b>
{
    from: &'a Vertex,
    to: &'b Vertex,
    latency: u32,
    throughput: u32,
}

impl<'a,'b> Edge<'a,'b>
{
    pub fn new(from: &'a Vertex, to: &'b Vertex) -> Edge<'a,'b>
    {
        Edge {
            from: from,
            to: to,
            latency: 0,
            throughput: 0
        }
    }
}

fn main() {

}
