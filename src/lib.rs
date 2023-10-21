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
{
    pub fn new() -> Self {
        return List {
            tail: None,
            head: None,
        };
    }

    pub fn add_node(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
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

        if self.find_path_recursive(&start, &end, &mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    fn find_path_recursive(&self, current: T, end: T, visited: &mut List<T>, path: &mut List<T>) -> bool {
        if visited.contains(current) {
            return false;
        }

        visited.add_node(current);
        path.add_node(current);

        if current == end {
            return true;
        }

        let mut current_node = &self.head;
        while let Some(node) = current_node {
            if node.value == current {
                // Encuentra el nodo actual
                if let Some(next) = &node.next {
                    if self.find_path_recursive(next.value.clone(), end.clone(), visited, path) {
                        return true;
                    }
                }
            }
            current_node = &node.next;
        }
        false
    }
}
