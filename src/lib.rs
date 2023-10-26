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

    pub fn add_node(&mut self, item: T){
        let new_node = Box::new(Node {
            value: item,
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
    pub fn add_edge(&mut self, from: usize, to: usize){
        let mut current = &self.head;
        let mut index = 0;

        while let Some(node) = current {
            if index == from {
                let mut current_to = &node.next;
                let mut index_to = 0;

                while let Some(node_to) = current_to{
                    if index_to == to{
                        return;
                    }

                    current_to = &node_to.next;
                    index_to += 1;
                }
                panic!("Node indices out of bounds.");
            }
            current = &node.next;
            index += 1;
        }
        panic!("Node indices out of bounds.");
    }
    
    pub fn contains(&self, item: T)->bool{
        let mut current = &self.head;

        while let Some(node) =current{
            if node.value == item {
                return true;
            }
            current = &node.next;
        }

        false
    }

}
