struct Neverzero(usize);

impl Neverzero {
    fn new(i: usize) -> Result<Self, String> {
        if i == 0 {
            Err("Cannot Be Zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divde(a: usize, b: Neverzero) -> usize {
    let b = b.0;
    a / b
}

pub fn main_new_feature() {
    match Neverzero::new(2) {
        Ok(val) => println!("{}", divde(2, val)),
        Err(e) => println!("{}", e),
    };
}
