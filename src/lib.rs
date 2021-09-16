#[allow(dead_code)]
mod graph {
    pub struct Vertex<T> {
        data: T,
    }

    impl<T> Vertex<T> {
        pub fn new(data: T) -> Vertex<T> {
            return Vertex { data };
        }

        pub fn data(&self) -> &T {
            return &self.data;
        }
    }

    pub struct Edge<T> {
        data: T,
        weight: u64,
    }

    impl<T> Edge<T> {
        pub fn new(data: T, weight: u64) -> Edge<T> {
            Edge { data, weight }
        }

        pub fn data(&self) -> &T {
            &self.data
        }

        pub fn weight(&self) -> u64 {
            self.weight
        }
    }

    pub struct GraphMatrix<Tv, Te> {
        vertex_collect: Vec<Vertex<Tv>>,
        edge_collect: Vec<Vec<Option<Edge<Te>>>>,
    }

    impl <Tv, Te> GraphMatrix<Tv, Te> {
        pub fn new() -> GraphMatrix<Tv, Te> {
            GraphMatrix {
                vertex_collect: Vec::new(),
                edge_collect: Vec::from(Vec::new()),
            }
        }

        pub fn insert_vertex(&mut self, data: Tv) {
            for head in self.edge_collect.iter_mut() {
                head.push(None);
            }

            self.vertex_collect.push(Vertex::new(data));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::{Edge, Vertex, GraphMatrix};

    #[test]
    fn test_new_vertex() {
        let vertex = Vertex::new(3);
        let want = 3;
        assert_eq!(want, *vertex.data());
    }

    #[test]
    fn test_new_edge() {
        let edge = Edge::new(10, 10);
        let want_data: i32 = 10;
        let want_weight: u64 = 10;

        assert_eq!(want_data, *edge.data());
        assert_eq!(want_weight, edge.weight());
    }

    #[test]
    fn test_new_graph_matrix() {
        let mut graph: GraphMatrix<i32, usize> = GraphMatrix::new();

        graph.insert_vertex(10);
    }
}
