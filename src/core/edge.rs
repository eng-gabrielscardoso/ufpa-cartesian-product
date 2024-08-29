use super::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Edge {
  pub source: Vertex,
  pub target: Vertex,
}

impl Edge {
  pub fn new(source: Vertex, target: Vertex) -> Self {
    Edge {
      source,
      target
    }
  }
}
