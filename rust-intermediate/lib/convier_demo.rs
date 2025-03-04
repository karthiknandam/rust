use std::path::Iter;

pub fn convier() {
    let mut convier_belt: ConvierBelt<ConvierItem> = ConvierBelt::new();
    convier_belt.add(ConvierItem::default());
    convier_belt.add(ConvierItem::default());
    convier_belt.add(ConvierItem::default());
    convier_belt.add(ConvierItem::default());

    for i in convier_belt.iter() {
        println!("{:?}", i.dimesions());
        println!("Height is {}", i.height());
    }
}

struct ConvierItem {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    item_id: String,
}

impl Default for ConvierItem {
    fn default() -> Self {
        Self {
            width: 21.0,
            height: 2.2,
            depth: 5.5,
            weight: 22.0,
            item_id: "hello".to_owned(),
        }
    }
}
#[derive(Debug)]
struct Dimesions {
    width: f64,
    height: f64,
    depth: f64,
}

trait ConveyTrait {
    fn height(&self) -> f64;
    fn dimesions(&self) -> Dimesions;
}

impl ConveyTrait for ConvierItem {
    fn dimesions(&self) -> Dimesions {
        return Dimesions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        };
    }
    fn height(&self) -> f64 {
        return self.height;
    }
}

struct ConvierBelt<T: ConveyTrait> {
    items: Vec<T>,
}

impl<T> ConvierBelt<T>
where
    T: ConveyTrait,
{
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add(&mut self, t: T) {
        self.items.push(t);
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}
