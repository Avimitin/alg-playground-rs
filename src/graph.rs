pub struct Vertex<T> {
    data: T,
}

impl<T> Vertex<T> {
    pub fn new(data: T) -> Vertex<T> {
        Vertex { data }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

// Edge contains data and weight for each connected vertex
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

impl <Tv, Te: Copy> GraphMatrix<Tv, Te>{
    pub fn new() -> GraphMatrix<Tv, Te> {
        GraphMatrix {
            vertex_collect: Vec::new(),
            edge_collect: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, data: Tv) {
        for head in self.edge_collect.iter_mut() {
            head.push(None);
        }

        self.edge_collect.push(vec![None]);

        self.vertex_collect.push(Vertex::new(data));
    }

    pub fn insert_edge(&mut self, i: usize, j: usize, edge: Edge<Te>) -> Result<(), String> {
        if !self.edge_valid(i, j) {
            return Err(format!("V = ({}, {}) is not valid", i, j));
        }

        if self.edge_exist(i, j) {
            return Err(format!("E = ({}, {}) is already exist", i, j));
        }

        self.edge_collect[i][j] = Some(edge);

        Ok(())
    }

    fn edge_exist(&self, i: usize, j: usize) -> bool {
        match self.edge_collect[i][j] {
            None => false,
            Some(_) => true,
        }
    }

    fn edge_valid(&self, i: usize, j: usize) -> bool {
        match self.edge_collect.get(i) {
            None => false,
            Some(inner) => match inner.get(j) {
                None => false,
                Some(_) => true,
            },
        }
    }
}

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

    graph.add_vertex(10);
}

#[test]
fn test_insert_edge() {
    let mut graph = GraphMatrix::new();

    // insert vertex
    graph.add_vertex(10);

    match graph.insert_edge(0, 1, Edge::new(1, 1)) {
        Ok(_) => panic!("No error return when accessing not exist edge"),
        Err(_) => (),
    }

    graph.add_vertex(10);
    match graph.insert_edge(0, 1, Edge::new(1, 1)) {
        Ok(_) => (),
        Err(e) => panic!("fail to insert edge: {}", e),
    }
}
