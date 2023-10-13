pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node { 
            data: 0, 
            next: None 
        }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    tail: *mut Node,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None,  tail: std::ptr::null_mut() }
    }

    pub fn push_left(&mut self, new_data: i32) {
        let new_node = Box::new(
            Node {
                data: new_data,
                next: self.head.take()
            }
        );

        if self.tail.is_null() {
            self.tail = &*new_node as *const _ as *mut _;
        }

        self.head = Some(new_node);
    }

    pub fn push_right(&mut self, new_data: i32) {
        let new_node = Box::new(
            Node {
                data: new_data,
                next: None
            }
        );

        let raw_node = &*new_node as *const _ as *mut _;        
        
        if self.head.is_none() {
            self.head = Some(new_node);
            self.tail = raw_node;
        } else {
            unsafe {
                (*self.tail).next = Some(new_node);
            }
            self.tail = raw_node;
        }
    }

    pub fn insert(&mut self, new_data: i32, pos: usize) {
        let mut current = &mut self.head;
        let mut current_pos = 0;

        while let Some(ref mut node) = current {
            if current_pos == pos {
                let new_node = Box::new(
                    Node {
                        data: new_data,
                        next: node.next.take()
                    }
                );
                node.next = Some(new_node);
                return;
            }
            current = &mut node.next;
            current_pos += 1;
        }

        return;
    }

    pub fn locate(&self, new_data: i32) -> String {
        let mut current = &self.head;
        let mut pos = 0;

        if self.head.is_none() {
            return "Not Found".to_owned();
        }

        while let Some(ref node) = *current {
            if node.data == new_data {
                return "Found on ".to_owned() + &pos.to_string();
            }
            current = &node.next;
            pos += 1;
        }
        return "Not Found".to_owned();
    }

    pub fn retrieve(&self, pos: usize) -> Option<i32> {
        let mut current = &self.head;
        let mut current_pos = 0;

        if self.head.is_none() {
            return None;
        }

        while let Some(ref node) = *current {
            if current_pos == pos {
                return Some(node.data);
            }
            current = &node.next;
            current_pos += 1;
        }
        return None;
    }

    pub fn delete(&mut self, new_data: i32) {
        if self.head.is_none() {
            return;
        }

        if self.head.as_ref().unwrap().data == new_data {
            self.head = self.head.take().unwrap().next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            return;
        }

        let mut current = &mut self.head;

        while let Some(ref mut node) = *current {
            if node.next.is_some() && node.next.as_ref().unwrap().data == new_data {
                if node.next.as_ref().unwrap().next.is_none() {
                    self.tail = &mut **node as *mut _;
                }
                node.next = node.next.take().unwrap().next;
                return;
            }
            current = &mut node.next;
        }
    }

    pub fn first(&self) -> Option<i32> {
        let first = &self.head.as_ref().unwrap().data;
        
        if !self.head.is_none() {
            return Some(first.clone());
        }

        None
    }

    pub fn find_max(&self) -> Option<i32> {
        let mut current = &self.head;
        let mut max = std::i32::MIN;

        if self.head.is_none() {
            return None;
        }

        while let Some(ref node) = *current {
            if node.data > max {
                max = node.data;
            }
            current = &node.next;
        }
        return Some(max);
    }

    pub fn del_one_after_max(&mut self) {
        let max = match self.find_max() {
            Some(val) => val,
            None => return,
        };
    
        let mut current = &mut self.head;
        while let Some(ref mut node) = *current {
            if node.data == max {
                node.next = node.next.take().and_then(|next_node| next_node.next);
                return;
            }
            current = &mut node.next;
        }
    }

    pub fn makenull(&mut self) {
        self.head = None;
        self.tail = std::ptr::null_mut();
    }

    pub fn print(&self) {
        let mut current = &self.head;
        if !self.head.is_none(){
            while let Some(ref node) = *current {
                println!("Data: {}", node.data);
                current = &node.next;
            }
        } else { println!("None"); }
    }
}

fn main(){
    let num1 = 1;
    let num2 = 2;
    let num3 = 3;
    let num5 = 5;

    let mut list: LinkedList = LinkedList::new();

    list.push_left(num1);
    list.push_right(num2);
    list.push_left(num3);

    list.print();

    print!("\nRun locate!\n{}\n\n", list.locate(num2));

    let num4 = list.retrieve(1);
    if num4 == None {
        print!("Not Found!\n");
    } else {
        let num4 = num4.unwrap();
        print!("Run retrieve!\n{}\n\n", num4);
    }

    list.delete(num2);
    print!("Run delete!\n");

    list.print();

    let first = list.first();
    if first == None {
        print!("Not Found!\n");
    } else {
        let first = first.unwrap();
        print!("\nPrinted first!\n{}", first);
    }

    list.insert(num5, 0);
    print!("\n\nRun insert!\n");
    list.print();

    list.del_one_after_max();
    print!("\n\nRun del_one_after_max!\n");
    list.print();

    list.makenull();
    print!("\n\nRun makenull!\n");
    list.print();
}