use uuid::Uuid;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq,Debug)]
#[allow(dead_code)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Nationality {
    UnitedStates,
    Australia,
    UnitedKingdom,
    Germany,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Skills {
    can_drive: bool,
    can_swim: bool,
    occupation: String,
    hobbies: Vec<String>
}

impl Skills {
    fn new() -> Skills {
        Skills {
            can_drive: false,
            can_swim: false,
            occupation: "None".to_string(),
            hobbies: vec![]
        }
    }
    fn set_can_swim(&mut self, value: bool) {
        // prevents setting the value to false
        if value {
            self.can_swim = value;
        }
    }

    fn set_can_drive(&mut self, value: bool) {
        // prevents setting the value to false
        if value {
            self.can_drive = value;
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Name {
    first_name: String,
    last_name: String,
    middle_name: String,
}

impl Name {
    fn new(f: String, m: String, l: String) -> Name {
        Name {
            first_name: f,
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
impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            names: Name {
                first_name: self.names.first_name.clone(),
                last_name: self.names.last_name.clone(),
                middle_name: self.names.middle_name.clone(),
            },
            uuid: self.uuid,
            age: self.age,
            gender: self.gender,
            nationality: self.nationality,
            friends: self.friends.clone(),
            family: self.family.clone(),
            skills: Skills {
                can_drive: self.skills.can_drive,
                can_swim: self.skills.can_swim,
                occupation: self.skills.occupation.clone(),
                hobbies: self.skills.hobbies.clone(),
            },
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
    pub fn new(
        first: String,
        middle: String,
        last: String,
        a: u32,
        gend: Gender,
        nation: Nationality,
    ) -> Person {
        Person {
            names: Name::new(first, middle, last),
            uuid: uuid::Uuid::new_v4(),
            age: a,
            gender: gend,
            nationality: nation,
            friends: vec![],
            family: vec![],
            skills: Skills::new(),
        }
    }
    pub fn get_job(&self)-> String{
        self.skills.occupation.clone()
    }

    pub fn get_hobbies(&self) -> Vec<String>{
        self.skills.hobbies.clone()
    }

    pub fn change_job(&mut self, new_job: String){
        self.skills.occupation = new_job;
    }

    pub fn quit_job(&mut self){
        self.skills.occupation = "None".to_string()
    }

    pub fn add_hobby(&mut self, hobby: String) {
        if ! self.skills.hobbies.contains(&hobby){
            self.skills.hobbies.push(hobby);
        }
    }

    pub fn remove_hobby(&mut self, hobby: String) {
        self.skills.hobbies.retain(|hob| hob != &hobby)

    }
    pub fn remove_friend(&mut self, person: &Person) {
        self.friends.retain(|x| {
            x != &StoragePerson {
                uuid: person.uuid,
                name: person.get_full_name(),
            }
        })
    }

    pub fn remove_family(&mut self, person: &Person) {
        self.family.retain(|x| {
            x != &StoragePerson {
                uuid: person.uuid,
                name: person.get_full_name(),
            }
        })
    }
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }
    pub fn can_drive(&self) -> bool {
        self.skills.can_drive
    }
    pub fn can_swim(&self) -> bool {
        self.skills.can_swim
    }

    pub fn learn_to_swim(&mut self) {
        self.skills.set_can_swim(true);
    }
    pub fn learn_to_drive(&mut self) {
        self.skills.set_can_drive(true);
    }

    pub fn add_friend(&mut self, friend: &Person) {
        let friend_store = StoragePerson {
            uuid: friend.uuid,
            name: friend.get_full_name(),
        };
        // prevents added duplicate people
        if !self.friends.contains(&friend_store) {
            self.friends.push(friend_store);
        }
    }

    pub fn add_family(&mut self, family: &Person) {
        let family_store = StoragePerson {
            uuid: family.uuid,
            name: family.get_full_name(),
        };
        // prevents added duplicate people
        if !self.family.contains(&family_store) {
            self.family.push(family_store);
        }
    }

    pub fn is_friend(&self, friend: &Person) -> bool {
        self.friends.contains(&StoragePerson {
            uuid: friend.uuid,
            name: friend.get_full_name(),
        })
    }

    pub fn is_family(&self, family: &Person) -> bool {
        self.family.contains(&StoragePerson {
            uuid: family.uuid,
            name: family.get_full_name(),
        })
    }

    pub fn get_first_name(&self) -> String {
        self.names.first_name.clone()
    }

    pub fn get_full_name(&self) -> String {
        format!(
            "{} {} {}",
            self.names.first_name, self.names.middle_name, self.names.last_name
        )
    }

    pub fn get_gender(&self) -> Gender {
        self.gender
    }

    pub fn get_nationality(&self) -> Nationality {
        self.nationality
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct StoragePerson {
    uuid: Uuid,
    name: String,
}

impl Clone for StoragePerson {
    fn clone(&self) -> Self {
        StoragePerson {
            uuid: self.uuid,
            name: self.name.clone(),
        }
    }
}
