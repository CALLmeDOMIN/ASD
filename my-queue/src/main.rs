pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: i32) -> Node {
        Node { data: data, next: None }
    }
}

pub struct MyQueue {
    pub front: Option<Box<Node>>,
    pub rear: *mut Node,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue { front: None, rear: std::ptr::null_mut() }
    }

    pub fn enqueue(&mut self, data: i32) {
        let new_node = Box::new(Node::new(data));
        let raw_node = &*new_node as *const _ as *mut _;

        if self.front.is_none() {
            self.front = Some(new_node);
            self.rear = raw_node;
        } else {
            unsafe {
                (*self.rear).next = Some(new_node);
            }
            self.rear = raw_node;
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        self.front.take().map(|old_front| {
            self.front = old_front.next;
            if self.front.is_none() {
                self.rear = std::ptr::null_mut();
            }
            old_front.data
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.front.as_ref().map(|node| node.data)
    }

    pub fn print(&self) {
        let mut current = &self.front;
        while let Some(ref node) = *current {
            println!("{}", node.data);
            current = &node.next;
        }
    }
}

pub fn funkcja(queue: &mut MyQueue) {
    for i in 1..21 {
        if i % 3 == 0 {
        queue.enqueue(i);
        }
    }
}

fn main() {
    let mut queue = MyQueue::new();

    funkcja(&mut queue);
    
    queue.print();

    println!("Peek: {}", queue.peek().unwrap());

    println!("Dequeue: {}", queue.dequeue().unwrap());

    queue.print();
}
