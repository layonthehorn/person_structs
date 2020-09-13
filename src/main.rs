use crate::person::{Gender, Nationality, Person};

mod person;

fn main() {
    let mut thomas = Person::new(
        "Thomas".to_string(),
        "Patrick".to_string(),
        "Dressel".to_string(),
        27,
        Gender::Male,
        Nationality::UnitedStates,
    );
    let mut darren = Person::new(
        "Darren".to_string(),
        "James".to_string(),
        "Capper".to_string(),
        27,
        Gender::Male,
        Nationality::Australia,
    );
    thomas.add_family(&darren);
    thomas.change_job("IT Helpdesk".to_string());
    thomas.add_hobby("Programming".to_string());
    darren.add_family(&thomas);
    darren.add_hobby("Writing".to_string());
    darren.change_job("Rally Driver".to_string());
    darren.learn_to_drive();

    if thomas.is_family(&darren) {
        println!("Big Brother Darren")
    }
    if darren.is_family(&thomas) {
        println!("Little Brother Thomas")
    }

    if thomas.get_nationality() == Nationality::UnitedStates {
        println!("Thomas is American.")
    }

    if darren.get_gender() == Gender::Male {
        println!("Darren is a wonderful man.")
    }
    if darren.can_drive() {
        println!("Darren is the best driver.")
    }
    if !thomas.can_swim() {
        println!("Thomas still can't swim.")
    }
    println!("Thomas's SSN is {}", thomas.get_uuid());
    println!("Darren's CRN is {}", darren.get_uuid());

    let mut looking = true;
    let people_vec = vec![darren,thomas];
    while looking{
        println!("Who do you want to look at?\nEnter the first name.");
        looking_at_people(&people_vec);
        looking = still_looking();

    }
    println!("Goodbye!")

}
fn still_looking() -> bool{
    println!("Are you still looking? y/n");
    let temp = & mut String::new();
    std::io::stdin()
        .read_line(temp)
        .expect("Failed to read line.");
    let check:String = match temp.trim().parse(){
        Ok(t) => t,
        Err(_e)=> "n".to_string()
    };
    println!();
    check == "Y".to_string() || check == "y".to_string()


}

fn looking_at_people(people: &Vec<Person>){
    for entry in people.iter(){
        print!("{} ",entry.get_first_name())
    }
    println!();
    let name =& mut String::new();
    std::io::stdin()
        .read_line(name).expect("Failed to read line.");
    let first_name: String = match name.trim().parse(){
        Ok(t) =>t,
        Err(_e) => "".to_string()
    };
    println!();
    for entry in people.iter(){
        if entry.get_first_name() == first_name{
            println!("Full Name: {}", entry.get_full_name());
            println!("Nationality: {:?}", entry.get_nationality());
            println!("Gender: {:?}", entry.get_gender());
            println!("Job {}",entry.get_job());
            print!("Hobbies: ");
            for hobby in entry.get_hobbies().iter(){
                print!("{} ", hobby)
            }
            println!();

        }
    }
}