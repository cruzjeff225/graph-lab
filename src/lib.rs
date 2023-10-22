use std::fmt::Debug;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T>
where
    T: PartialEq<T>,
    T: Debug,
    T: Clone,

{
    pub fn new() -> Self {
        return List {
            tail: None,
            head: None,
        };
    }

    pub fn add_node(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value.to_owned(),
            next: None,

        });

        let raw_node = Box::into_raw(new_node);

        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(Box::from_raw(raw_node));
            }
        } else {
            self.head = unsafe { Some(Box::from_raw(raw_node)) }
        }

        self.tail = Some(raw_node);
    }

     pub fn add_edge(&mut self, from: usize, to: usize) {
        let from_node = self.get_node_at_index(from);
        let to_node = &self.get_node_at_index(to);

        if &from_node.is_none() || to_node.is_none() {
            panic!("Node indices out of bounds.");
        }

        let from_node = from_node.unwrap();
        let to_node = to_node.unwrap();

        from_node.next = Some(Box::new(Node {
            value: to_node.value.clone(),
            next: from_node.next.take(),
        }));

        to_node.next = Some(Box::new(Node {
            value: from_node.value.clone(),
            next: to_node.next.take(),
        }));
    }

    fn get_node_at_index(&mut self, index: usize) -> Option<&mut Node<T>> {
        let mut current = &mut self.head;
        let mut i = 0;

        while let Some(ref mut node) = *current {
            if i == index {
                return Some(node);
            }
            i += 1;
            current = &mut node.next;
        }

        None
    }


    pub fn contains(&self, value: T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == value {
                return true;
            }
            current = &node.next;
        }
        false
    }

    pub fn find_path(&self, start: T, end: T) -> Option<List<T>> {
        let mut visited = List::new();
        let mut path = List::new();
    
        if self.find_path_recursive(&start, end, &mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }
    
    fn find_path_recursive(&self, current: &T, end: T, visited: &mut List<T>, path: &mut List<T>) -> bool {
        if visited.contains(current.to_owned()) {
            return false;
        }
    
        visited.add_node(current.to_owned());
        path.add_node(current.to_owned());
    
        if *current == end {
            return true;
        }
    
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            if node.value == *current {
                // Encuentra el nodo actual
                if let Some(next) = &node.next {
                    if self.find_path_recursive(&next.value, end.clone(), visited, path) {
                        return true;
                    }
                }
            }
            current_node = &node.next;
        }
        false
    }
}
