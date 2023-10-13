struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

pub struct MyStack {
    top: Option<Box<Node>>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack { top: None }
    }

    pub fn push(&mut self, new_data: i32) {
        let new_node = Box::new(
            Node {
                data: new_data,
                next: self.top.take(),
            }
        );
        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.top.take().map(|node| {
            self.top = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.top.as_ref().map(|node| node.data)
    }

    pub fn print(&self) {
        let mut current = &self.top;
        while let Some(ref node) = *current {
            print!("{} ", node.data);
            current = &node.next;
        }
        println!();
    }
}

fn main() {
    let mut stack = MyStack::new();

    let num1 = 1;
    let num2 = 2;
    let num3 = 3;

    stack.push(num1);
    stack.push(num2);
    stack.push(num3);

    stack.print();

    println!("Top element is: {}", stack.peek().unwrap());

    println!("Popped element is: {}", stack.pop().unwrap());

    stack.print();
}