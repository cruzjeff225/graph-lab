use std::fmt::Debug;

struct Node<T> {
    value: T,
    next: Option <Box<Node<T>>>,
}
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    tail: Option <*mut Node<T>>,
}

///pub struct List <T>{
    ///head: Option<Box<Node<T>>>,
    ///tail: Option<*mut Node<T>>,
///}

impl<T> List<T>
where
    T: PartialEq<T>,
    T: Debug,

{
    pub fn new() -> Self {
        return List {
            tail: None,
            head: None,
        };
    }

    pub fn add_node(&mut self, value: T) {
        let new_node= Box::new(Node{
            value,
            next: None,
        });
            
        let raw_node = Box::into_raw(new_node);

        match self.tail {
            Some(tail_ptr) => {
                unsafe {
                    (*tail_ptr).next = Some(Box::from_raw(raw_node));
                }
            }
            None => {
                self.head = Some(Box::from_raw(raw_node));
            }
        }

        self.tail = Some(raw_node);
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