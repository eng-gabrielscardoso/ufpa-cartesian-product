use super::vertex::Vertex;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Edge {
  pub source: Vertex,
  pub target: Vertex,
}

impl Edge {
  pub fn new(source: &Vertex, target: &Vertex) -> Self {
    Edge {
      source: source.clone(),
      target: target.clone(),
    }
  }
}
