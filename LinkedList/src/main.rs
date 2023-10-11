pub struct Person {
    pub name: Box<str>,
    pub surname: Box<str>,
    pub age: Box<i32>,
    pub next: Option<Box<Person>>,
}

pub struct LinkedList {
    head: Option<Box<Person>>,
    tail: *mut Person,
}

impl Person {
    pub fn new() -> Self {
        Person { 
            name: "".into(), 
            surname: "".into(), 
            age: 0.into(), 
            next: None 
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.surname == other.surname && self.age == other.age
    }
}

impl LinkedList
where Person: PartialEq {
    pub fn new() -> Self {
        LinkedList { head: None,  tail: std::ptr::null_mut() }
    }

    pub fn push_left(&mut self, person: Person) {
        let new_node = Box::new(
            Person { 
                name: person.name, 
                surname: person.surname, 
                age: person.age,
                next: self.head.take()
            }
        );

        if self.tail.is_null() {
            self.tail = &*new_node as *const _ as *mut _;
        }

        self.head = Some(new_node);
    }

    pub fn push_right(&mut self, person: &Person) {
        let new_node = Box::new(
            Person {
                name: person.name.clone(),
                surname: person.surname.clone(),
                age: person.age.clone(),
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

    pub fn locate(&self, person: &Person) -> String {
        let mut current = &self.head;
        let mut pos = 0;

        if self.head.is_none() {
            return "Not Found".to_owned();
        }

        while let Some(ref node) = *current {
            if **node == *person {
                return "Found on ".to_owned() + &pos.to_string();
            }
            current = &node.next;
            pos += 1;
        }
        return "Not Found".to_owned();
    }

    pub fn retrieve(&self, pos: usize) -> Option<&Person> {
        let mut current = &self.head;
        let mut current_pos = 0;

        if self.head.is_none() {
            return None;
        }

        while let Some(ref node) = *current {
            if current_pos == pos {
                return Some(node);
            }
            current = &node.next;
            current_pos += 1;
        }
        return None;
    }

    pub fn delete(&mut self, person: &Person) {
        if self.head.is_none() {
            return;
        }

        if **self.head.as_ref().unwrap() == *person {
            self.head = self.head.take().unwrap().next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            return;
        }

        let mut current = &mut self.head;

        while let Some(ref mut node) = *current {
            if node.next.is_some() && **node.next.as_ref().unwrap() == *person {
                if node.next.as_ref().unwrap().next.is_none() {
                    self.tail = &mut **node as *mut _;
                }
                node.next = node.next.take().unwrap().next;
                return;
            }
            current = &mut node.next;
        }
    }

    pub fn print(&self) {
        let mut current = &self.head;
        if !self.head.is_none(){
            while let Some(ref node) = *current {
                println!("Person: {} {} {}", node.name, node.surname, node.age);
                current = &node.next;
            }
        } else { println!("None"); }
    }
}

fn main(){
    let person: Person = Person {
        name: "John".into(),
        surname: "Doe".into(),
        age: 42.into(),
        next: None
    };

    let person2: Person = Person {
        name: "Jan".into(),
        surname: "Kowalski".into(),
        age: 20.into(),
        next: None
    };

    let person3: Person = Person {
        name: "Maciej".into(),
        surname: "Nowak".into(),
        age: 30.into(),
        next: None
    };

    let mut list: LinkedList = LinkedList::new();

    list.push_left(person);
    list.push_right(&person2);
    list.push_left(person3);

    list.print();

    print!("\n{}", list.locate(&person2));

    let person = list.retrieve(1).unwrap();

    print!("\n\n{} {} {}", person.name, person.surname, person.age);

    list.delete(&person2);

    print!("\n\n");

    list.print();
}