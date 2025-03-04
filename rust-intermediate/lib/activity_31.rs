// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    fn price(&self) -> f32;
}
struct Carpet {
    price: f32,
    length: f32,
}
struct Tile {
    price: f32,
    length: f32,
}
struct Wood {
    price: f32,
    length: f32,
}

impl Material for Carpet {
    fn price(&self) -> f32 {
        self.price * self.length
    }
}

impl Material for Tile {
    fn price(&self) -> f32 {
        self.price * self.length
    }
}

impl Material for Wood {
    fn price(&self) -> f32 {
        self.price * self.length
    }
}
struct Materials {
    list: Vec<Box<dyn Material>>,
}
impl Materials {
    fn new() -> Self {
        Self { list: Vec::new() }
    }
    fn add<T: Material + 'static>(&mut self, t: T) {
        self.list.push(Box::new(t));
    }
    fn total(&self) -> Option<f32> {
        let total_price = self.list.iter().map(|f| f.price()).reduce(|a, b| a + b);
        return total_price;
    }
}
pub fn main_act_31() {
    let mut value = Materials::new();
    value.add(Wood {
        price: 15.0,
        length: 2.0,
    });
    value.add(Tile {
        price: 15.0,
        length: 2.0,
    });
    value.add(Carpet {
        price: 15.0,
        length: 2.0,
    });
    match value.total() {
        Some(val) => println!("Total value is {}", val),
        None => println!("None"),
    };
}
//* version 2 implimentation;

trait Material2 {
    fn cost_per_meter(&self) -> f64;
    fn square_meter(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.cost_per_meter() * self.square_meter()
    }
}

struct Cloth2(f64);
struct Wood2(f64);
struct Nylon(f64);
impl Material2 for Cloth2 {
    fn cost_per_meter(&self) -> f64 {
        12.0
    }
    fn square_meter(&self) -> f64 {
        self.0
    }
}
impl Material2 for Wood2 {
    fn cost_per_meter(&self) -> f64 {
        17.0
    }
    fn square_meter(&self) -> f64 {
        self.0
    }
}
impl Material2 for Nylon {
    fn cost_per_meter(&self) -> f64 {
        15.0
    }
    fn square_meter(&self) -> f64 {
        self.0
    }
}

fn total_cost(list: &Vec<Box<dyn Material2>>) -> f64 {
    list.iter().map(|f| f.total_cost()).sum()
}

pub fn test_v2_challenge() {
    let cloths: Vec<Box<dyn Material2>> = vec![
        Box::new(Cloth2(2.4)),
        Box::new(Wood2(3.2)),
        Box::new(Nylon(5.2)),
    ];
    let val = total_cost(&cloths);
    println!("Your total cost is {}", val);
}
