
struct  Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    
}


struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: u32,
}


impl<T>  LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    fn add(&mut self, value: T) {
        let node = Node { value, next: self.head.take() };
        self.head = Some(Box::new(node));
        self.len += 1;
    }
}

pub fn linked() {
    let mut linked_list: LinkedList<u32> = LinkedList::new();
    linked_list.add(1);
    linked_list.add(2);
    linked_list.add(3);

    let mut data = linked_list.head.as_ref();
    for i in 0..4 {
        match data{
            Some(node) =>  {
            println!("node {}: {}", i, node.value);
            data = node.next.as_ref();
            },
            None => print!("reach end of the list \n"),
        }


    }
    
}