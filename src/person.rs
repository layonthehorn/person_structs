pub struct Person {
    first_name: String,
    last_name: String,
    middle_name: String,
    age: u32,
    friends: Vec<Person>,
    family: Vec<Person>,
}

impl Person {
    pub fn new(first: String, middle: String, last: String, a: u32) -> Person {
        Person{
            first_name: first,
            last_name: last,
            middle_name: middle,
            age: a,
            friends: vec![],
            family: vec![]
        }

    }

    pub fn add_friend(&mut self,friend: Person) {
        self.friends.push(friend);

    }

    pub fn add_family(&mut self,family: Person) {
        self.family.push(family);

    }

    pub fn get_full_name(&self) -> String {
        format!("{} {} {}",self.first_name, self.middle_name, self.last_name)
    }

}
