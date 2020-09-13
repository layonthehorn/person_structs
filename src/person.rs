use uuid::Uuid;

#[derive(Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Gender{
    Male,
    Female
}

#[derive(Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
#[allow(dead_code)]
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

#[derive(Ord, PartialOrd, Eq)]
pub struct Person {
    names: Name,
    uuid: Uuid,
    age: u32,
    gender: Gender,
    nationality: Nationality,
    friends: Vec<StoragePerson>,
    family: Vec<StoragePerson>,
    skills: Skills,
}

// allows cloning Person traits
impl Clone for Person{
    fn clone(&self) -> Self {
        Person{
            names: Name {
                first_name: self.names.first_name.clone(),
                last_name: self.names.last_name.clone(),
                middle_name: self.names.middle_name.clone()
            },
            uuid: self.uuid,
            age: self.age,
            gender: self.gender,
            nationality: self.nationality,
            friends: self.friends.clone(),
            family: self.family.clone(),
            skills: Skills { can_drive: self.skills.can_drive, can_swim: self.skills.can_swim }
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
       self.uuid == other.uuid
    }
}
#[allow(dead_code)]
impl Person {
    pub fn new(first: String, middle: String, last: String, a: u32, gend: Gender, nation: Nationality) -> Person {
        Person{
            names: Name::new(first,middle,last),
            uuid: uuid::Uuid::new_v4(),
            age: a,
            gender: gend,
            nationality: nation,
            friends: vec![],
            family: vec![],
            skills: Skills::new()

        }

    }
    pub fn get_uuid(&self) -> Uuid{
        self.uuid
    }
    pub fn can_drive(&self) -> bool {
        self.skills.can_drive
    }
    pub fn can_swim(&self) -> bool {
        self.skills.can_swim
    }

    pub fn learn_to_swim(&mut self){
        self.skills.set_can_swim(true);
    }
    pub fn learn_to_drive(&mut self){
        self.skills.set_can_drive(true);
    }

    pub fn add_friend(&mut self,friend: &Person) {
        self.friends.push(StoragePerson{
            uuid:friend.uuid,
            name: friend.get_full_name()
        });

    }

    pub fn add_family(&mut self,family: &Person) {
        self.family.push(StoragePerson{
            uuid: family.uuid,
            name: family.get_full_name()
        });

    }

    pub fn is_friend(&self,friend: &Person) -> bool{
       self.friends.contains(&StoragePerson{
           uuid: friend.uuid,
           name: friend.get_full_name()
       })
    }

    pub fn is_family(&self,family: &Person ) -> bool {
        self.family.contains(&StoragePerson{
            uuid: family.uuid,
            name: family.get_full_name()
        })
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct StoragePerson{
    uuid: Uuid,
    name: String

}

impl Clone for StoragePerson{
    fn clone(&self) -> Self {
        StoragePerson{
            uuid: self.uuid,
            name: self.name.clone()
        }
    }
}