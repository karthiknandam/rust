// ^ This is used especially for polymorphic behaviours we can use single trait obj and make it in the head and iter through it using box::new() values DYN

trait Draw {
    fn print_val(&self);
}
struct Dimenstions {
    width: isize,
    height: isize,
}

struct Data {
    name: String,
    age: isize,
}

impl Draw for Data {
    fn print_val(&self) {
        println!("Name : {} Age : {}", self.name, self.age);
    }
}
impl Draw for Dimenstions {
    fn print_val(&self) {
        println!("Width : {} Height : {}", self.width, self.height);
    }
}

struct MultiVals {
    components: Vec<Box<dyn Draw>>,
}
impl MultiVals {
    fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    fn print_all_vals(&self) {
        for a in self.components.iter() {
            a.print_val();
        }
    }
    fn add<T: Draw + 'static>(&mut self, t: T) {
        self.components.push(Box::new(t));
    }
}

pub fn multi() {
    let mut new_vec = MultiVals::new();
    new_vec.add(Dimenstions {
        width: 22,
        height: 12,
    });
    new_vec.add(Data {
        name: "Karthikeya".to_owned(),
        age: 21,
    });
    new_vec.print_all_vals();
}
