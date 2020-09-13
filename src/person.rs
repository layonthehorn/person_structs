#[derive(Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
pub enum Gender{
    Male,
    Female
}

#[derive(Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
pub enum Nationality {
    UnitedStates,
    Australia,
    UnitedKingdom,
    Germany
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Skills{
    can_drive: bool,
    can_swim: bool

}

impl Skills{
    fn new() -> Skills{
        Skills{
            can_drive: false,
            can_swim: false
        }
    }
    fn set_can_swim(&mut self, value: bool){
        // prevents setting the value to false
        if value{
            self.can_swim = value;
        }
    }

    fn set_can_drive(&mut self, value: bool){
        // prevents setting the value to false
        if value{
            self.can_drive = value;
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Name{

    first_name: String,
    last_name: String,
    middle_name: String,
}

impl Name {
    fn new(f: String, m:String,l:String)-> Name{
        Name{
            first_name:f,
            middle_name: m,
            last_name: l,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Person {
    names: Name,
    age: u32,
    gender: Gender,
    nationality: Nationality,
    friends: Vec<Person>,
    family: Vec<Person>,
    skills: Skills,
}

impl Person {
    pub fn new(first: String, middle: String, last: String, a: u32, gend: Gender, nation: Nationality) -> Person {
        Person{
            names: Name::new(first,middle,last),
            age: a,
            gender: gend,
            nationality: nation,
            friends: vec![],
            family: vec![],
            skills: Skills::new()

        }

    }

    pub fn add_friend(&mut self,friend: Person) {
        self.friends.push(friend);

    }

    pub fn remove_family(&mut self, family: Person){
        self.family.retain(|fam| *fam != family)
    }

    pub fn remove_friend(&mut self, friend: Person) {
        self.friends.retain(|fre| *fre != friend)
    }

    pub fn add_family(&mut self,family: Person) {
        self.family.push(family);

    }

    pub fn is_friend(&self,friend: &Person) -> bool{
       self.friends.contains(friend)
    }

    pub fn is_family(&self,family: &Person ) -> bool {
        self.family.contains(family)
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {} {}",self.names.first_name, self.names.middle_name, self.names.last_name)
    }

    pub fn get_gender(&self) -> Gender {
        self.gender
    }

    pub fn get_nationality(&self) -> Nationality {
        self.nationality
    }

}
