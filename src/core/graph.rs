use std::collections::HashSet;

use super::{edge::Edge, vertex::Vertex};

#[derive(Debug, Clone)]
pub struct Graph {
    edges: HashSet<Edge>,
    vertices: HashSet<Vertex>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashSet::new(),
            vertices: HashSet::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge);
    }

    pub fn cartesian_product(graph_a: &Graph, graph_b: &Graph) -> Graph {
        let mut result = Graph::new();

        for source in &graph_a.vertices {
            for target in &graph_b.vertices {
                let combined_vertex = Vertex::new(&format!("({},{})", source.get_id(), target.get_id()));
                result.add_vertex(combined_vertex);
            }
        }

        for source in &graph_a.vertices {
            for target in &graph_b.vertices {
                let source_id = source.get_id();
                let target_id = target.get_id();

                for v1_adj in &graph_a.vertices {
                    if v1_adj == source || !graph_a.edges.contains(&Edge::new(source, v1_adj)) {
                        continue;
                    }
                    let combined_vertex_u = Vertex::new(&format!("({},{})", source_id, target_id));
                    let combined_vertex_v = Vertex::new(&format!("({},{})", v1_adj.get_id(), target_id));
                    result.add_edge(Edge::new(&combined_vertex_u, &combined_vertex_v));
                }

                for v2_adj in &graph_b.vertices {
                    if v2_adj == target || !graph_b.edges.contains(&Edge::new(target, v2_adj)) {
                        continue;
                    }
                    let combined_vertex_u = Vertex::new(&format!("({},{})", source_id, target_id));
                    let combined_vertex_v = Vertex::new(&format!("({},{})", source_id, v2_adj.get_id()));
                    result.add_edge(Edge::new(&combined_vertex_u, &combined_vertex_v));
                }
            }
        }

        result
    }
}
