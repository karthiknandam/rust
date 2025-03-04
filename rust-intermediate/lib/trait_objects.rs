pub fn main_trait() {
    let mouse = Box::new(Mouse);
    // let mouser: &dyn Clicky = &Mouse;
    // * Vector Creation using the values of the mouses;

    let vector_mouse = Box::new(Mouse);
    let vector_mouse1 = Box::new(Mouse);

    let vec_values: Vec<Box<dyn Clicky>> = vec![vector_mouse, vector_mouse1];
    enter_dyn_array(vec_values);
    enter_dyn(mouse);
}

fn enter_dyn_array(arr: Vec<Box<dyn Clicky>>) {
    for a in arr {
        a.hello();
    }
}

fn enter_dyn(mouse: Box<dyn Clicky>) {
    mouse.hello();
}

// Type annotation and dynamic allocation of the trait;

struct Mouse;
impl Clicky for Mouse {
    fn hello(&self) {
        println!("Clickkkkk....");
    }
}
trait Clicky {
    fn hello(&self);
}
