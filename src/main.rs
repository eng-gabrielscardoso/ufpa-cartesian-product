use core::{edge::Edge, graph::Graph, vertex::Vertex};

mod core;

fn main() {
    let mut graph_a = Graph::new();
    let mut graph_b = Graph::new();

    let vertex_a = Vertex::new("A");
    let vertex_b = Vertex::new("B");
    let vertex_c = Vertex::new("C");
    let vertex_d = Vertex::new("D");

    let edge_a = Edge::new(&vertex_a, &vertex_b);
    let edge_b = Edge::new(&vertex_c, &vertex_d);

    graph_a.add_vertex(vertex_a);
    graph_a.add_vertex(vertex_b);
    graph_a.add_edge(edge_a);

    graph_b.add_vertex(vertex_c);
    graph_b.add_vertex(vertex_d);
    graph_b.add_edge(edge_b);

    let cartesian_product = Graph::cartesian_product(&graph_a, &graph_b);

    println!("{:?}", cartesian_product);
}
