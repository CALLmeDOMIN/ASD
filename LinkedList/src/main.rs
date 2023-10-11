pub struct Person {
    pub name: Box<str>,
    pub surname: Box<str>,
    pub age: Box<i32>,
}

impl Person {
    pub fn new() -> Self {
        Person { 
            name: "".into(), 
            surname: "".into(), 
            age: 0.into(),
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.surname == other.surname && self.age == other.age
    }
}

pub struct Node {
    pub person: Person,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node { 
            person: Person::new(), 
            next: None 
        }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    tail: *mut Node,
}

impl LinkedList
where Person: PartialEq {
    pub fn new() -> Self {
        LinkedList { head: None,  tail: std::ptr::null_mut() }
    }

    pub fn push_left(&mut self, person: Person) {
        let new_node = Box::new(
            Node {
                person: Person {
                    name: person.name.clone(),
                    surname: person.surname.clone(),
                    age: person.age.clone(),
                },
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
            Node {
                person: Person {
                    name: person.name.clone(),
                    surname: person.surname.clone(),
                    age: person.age.clone(),
                },
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
            if node.person == *person {
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
                return Some(&node.person);
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

        if self.head.as_ref().unwrap().person == *person {
            self.head = self.head.take().unwrap().next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            return;
        }

        let mut current = &mut self.head;

        while let Some(ref mut node) = *current {
            if node.next.is_some() && node.next.as_ref().unwrap().person == *person {
                if node.next.as_ref().unwrap().next.is_none() {
                    self.tail = &mut **node as *mut _;
                }
                node.next = node.next.take().unwrap().next;
                return;
            }
            current = &mut node.next;
        }
    }

    pub fn first(&self) -> Option<Person> {
        let first = &self.head.as_ref().unwrap().person;
        
        if !self.head.is_none() {
            return Some(Person {
                name: first.name.clone(),
                surname: first.surname.clone(),
                age: first.age.clone(),
                }
            )
        }

        None
    }

    pub fn print(&self) {
        let mut current = &self.head;
        if !self.head.is_none(){
            while let Some(ref node) = *current {
                println!("Person: {} {} {}", node.person.name, node.person.surname, node.person.age);
                current = &node.next;
            }
        } else { println!("None"); }
    }
}

fn main(){
    // let person: Person = Person {
    //     name: "John".into(),
    //     surname: "Doe".into(),
    //     age: 42.into()
    // };

    let person2: Person = Person {
        name: "Jan".into(),
        surname: "Kowalski".into(),
        age: 20.into(),
    };

    // let person3: Person = Person {
    //     name: "Maciej".into(),
    //     surname: "Nowak".into(),
    //     age: 30.into(),
    // };

    let mut list: LinkedList = LinkedList::new();

    // list.push_left(person);
    // list.push_right(&person2);
    // list.push_left(person3);

    list.print();

    print!("\nRun locate!\n{}\n\n", list.locate(&person2));

    let person3 = list.retrieve(1);
    if person3 == None {
        print!("Not Found!\n");
    } else {
        let person3 = person3.unwrap();
        print!("Run retrieve!\n{} {} {}\n\n", person3.name, person3.surname, person3.age);
    }

    list.delete(&person2);
    print!("Run delete!\n");

    list.print();

    let first = list.first();
    if first == None {
        print!("Not Found!\n");
    } else {
        let first = first.unwrap();
        print!("\nPrinted first!\n{} {} {}", first.name, first.surname, first.age);
    }
}