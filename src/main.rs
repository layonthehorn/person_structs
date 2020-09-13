use crate::person::{Person, Gender, Nationality};

mod person;

fn main() {
    let mut thomas = Person::new("Thomas".to_string(), "Patrick".to_string(), "Dressel".to_string(),27, Gender::Male, Nationality::UnitedStates);
    let mut darren = Person::new("Darren".to_string(), "Lion".to_string(), "Capper".to_string(),27, Gender::Male, Nationality::Australia);
    thomas.add_family(&darren);
    darren.add_family(&thomas);

    if thomas.is_family(&darren){
        println!("Big Brother Darren")
    }
    if darren.is_family(&thomas){
        println!("Little Brother Thomas")
    }

}


