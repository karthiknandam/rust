use std::{collections::HashMap, io, vec};
// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter the value again")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
fn get_amount_input() -> Option<f32>{ 
    loop {
        let input = get_input()?;
        match input.parse::<f32>() {
            Ok(res) => return Some(res),
            Err(_) => println!("Plese Enter the correct number"),
        };
    }
}

enum ManageBill {
    AddBill,
    ViewBill,
    Remove,
    Update
}
impl ManageBill {
    fn from_str(input: &str) -> Option<ManageBill> {
        match input {
            "1" => Some(ManageBill::AddBill),
            "2" => Some(ManageBill::ViewBill),
            "3" => Some(ManageBill::Remove),
            "4" => Some(ManageBill::Update),
            _ => None,
        }
    }
    fn show() {
        println!("== Manage Bill ==\n 1. Add Bill \n 2. View Bill \n 3. Remove Bill \n 4. Update Bill \n 5. Exit");
    }
}
#[derive(Debug , Clone)]
struct Bill {
    name: String,
    amount: f32,
}

struct Bills{ 
    innner : HashMap<String,Bill>
}
impl Bills{ 
    fn new() -> Self{ 
        Self{
            innner : HashMap::new()
        }
    }
    fn add_bill(&mut self , bill : Bill){ 
        self.innner.insert(bill.name.clone(),bill);
    }
    fn get_bill(&self) -> Vec<&Bill> { 
        self.innner.values().collect()
    }
    fn remove_bill(&mut self , name : &str) -> bool{ 
        self.innner.remove(name).is_some()
    }
    fn update_bill(&mut self , name : &str , amount : f32)-> bool{ 
        match self.innner.get_mut(name){ 
            Some(bill) => { 
                bill.amount = amount;
                true
            },
            None => false
        }
    }
}

mod menu{
    use super::{get_amount_input, get_input, Bill, Bills};
 
    pub fn add(bills : &mut Bills){ 
        println!("Please Enter your bill name");
        let name : String = match get_input() {
            Some(inp) => inp,
            None => return
        };
        println!("Please Enter your Bill Amount");
        let amount : f32 = match get_amount_input(){ 
            Some(inp) => inp,
            None => return
        };
        bills.add_bill(Bill{name,amount});
        println!("Bill added Succesfully");
    }
    pub fn view_bill(bills : &Bills){ 
        for bill in bills.get_bill(){ 
            println!("{:?}" , bill);
        }
    }
    pub fn remove_bill(bills : &mut Bills){
        for bill in bills.get_bill(){ 
            println!("{:?}" , bill);
        }
        let name = match get_input(){ 
            Some(entry) => entry,
            None => return
        };
        if bills.remove_bill(name.as_str()){ 
            println!("Bill Removed");
        }
        else{ 
            println!("Bill not found");
        }
    }
    pub fn update_bill(bills : &mut Bills){ 
        for bill in bills.get_bill(){ 
            println!("{:?}", bill);
        }
        println!("Enter name of the bill to update");

        let name = match get_input() {
            Some(name) => name,
            None => return
        };
        println!("Enter the amount");

        let amount = match get_amount_input(){ 
            Some(amount) => amount,
            None => return
        };

        if bills.update_bill(name.as_str(), amount){ 
            println!("Updated Succesfully");
        }else{ 
            println!("Bill not found");
        }
    }
}

pub fn bill_task() -> Option<()> {
    use menu;
    let mut bills = Bills::new();
    loop {
        ManageBill::show();
        let input_value = get_input()?;
        match ManageBill::from_str(input_value.as_str()) {
            Some(ManageBill::AddBill) => menu::add(&mut bills),
            Some(ManageBill::ViewBill) => menu::view_bill(&bills),
            Some(ManageBill::Remove) => menu::remove_bill(&mut bills),
            Some(ManageBill::Update) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}
