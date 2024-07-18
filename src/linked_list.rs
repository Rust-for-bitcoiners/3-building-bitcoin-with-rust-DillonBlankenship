pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });
        
        match self.head {
            Some(ref mut head) => {
                let mut current = head;
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        } 
    }
}
