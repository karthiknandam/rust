// Section one working with lifetimes struct sei implimentation
#[derive(Debug)]
enum Course {
    Free,
    Paid,
}

#[derive(Debug)]
struct HarkiratCourse<'a> {
    course: &'a Course,
}
struct Courses {
    allcourses: Vec<Course>,
}

fn name<'a>(arg: &'a String) {
    let res: String = String::from("Noewww");
    let resu = arg.to_owned() + &res;
    println!("{resu}");
}

pub fn lifetimes() {
    let mut courses = Courses {
        allcourses: Vec::new(),
    };
    courses.allcourses.push(Course::Free);
    courses.allcourses.push(Course::Paid);

    {
        let val = HarkiratCourse {
            course: &courses.allcourses[0],
        };
        println!("{:?}", val);
    }
    {
        name(&"Hello ".to_owned());
    }
}
// Demo-Model;

#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City {
    Barland,
    Bazopolis,
    Fooville,
}

#[derive(Debug)]
struct IdCard {
    name: String,
    age: u8,
    city: City,
}

impl IdCard {
    pub fn new(name: &str, age: u8, city: City) -> Self {
        Self {
            name: name.to_string(),
            age,
            city,
        }
    }
}

fn new_ids() -> Cards {
    Cards {
        inner: vec![
            IdCard::new("Amy", 1, City::Fooville),
            IdCard::new("Matt", 10, City::Barland),
            IdCard::new("Bailee", 20, City::Barland),
            IdCard::new("Anthony", 30, City::Bazopolis),
            IdCard::new("Tina", 40, City::Bazopolis),
        ],
    }
}

struct Adults<'a> {
    inner: Vec<&'a IdCard>,
}

impl<'a> Adults<'a> {
    fn filter_by_place(&self, place: City) -> Vec<&IdCard> {
        self.inner
            .iter()
            .filter(|id| id.city == place)
            .map(|f| *f)
            .collect()
    }
}

pub fn test_2() {
    let ids: Cards = new_ids();

    let adults: Adults<'_> = Adults {
        inner: ids.inner.iter().filter(|id| id.age >= 20).collect(),
    };

    for id in ids.inner.iter() {
        println!("{:?}", id);
    }
    println!("\nAdults");
    for id in adults.inner.iter() {
        println!("{:?}", id);
    }
    println!("\nFilterd People");
    for id in adults.filter_by_place(City::Bazopolis).iter() {
        println!("{:?}", id);
    }
}
