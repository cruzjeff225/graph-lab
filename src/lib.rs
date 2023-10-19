use std::fmt::Debug;
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    value: T,
    neighbors: Vec<usize>,
}

impl<T> Graph<T>
where
    T: PartialEq<T>,
{
    pub fn new() -> Self {
        return Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.push(Node {
            value,
            neighbors: Vec::new(),
        });
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.nodes[from].neighbors.push(to);
            self.nodes[to].neighbors.push(from);
        } else {
            panic!("Node indices out of bounds.");
        }
    }

    pub fn contains(&self, value: T) -> bool {
        return self.nodes
            .iter()
            .find(|current| current.value == value)
            .is_some()
    }

    pub fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut visited = vec![false; self.nodes.len()];
        let mut path = Vec::new();

        if self.find_path_recursive(start, end, &mut visited, &mut path) {
            return Some(path)
        } else {
            return None
        }
    }

    fn find_path_recursive(
        &self,
        current: usize,
        end: usize,
        visited: &mut Vec<bool>,
        path: &mut Vec<usize>,
    ) -> bool {
        visited[current] = true;
        path.push(current);

        if current == end {
            return true;
        }

        for &neighbor in &self.nodes[current].neighbors {
            if !visited[neighbor] && self.find_path_recursive(neighbor, end, visited, path) {
                return true;
            }
        }

        path.pop();
        return false
    }
}