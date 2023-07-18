#[derive(PartialEq, Eq, Debug)]
pub struct Club {
    id: usize,
    name: String,
    description: String,
    max_members: usize,
    members: Vec<usize>,
}

impl Club {
    pub fn new(id: usize, name: String, description: String, max_members: usize) -> Club {
        Club {
            id,
            name,
            description,
            max_members,
            members: Vec::<usize>::new(),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_max_members(self) -> usize {
        self.max_members
    }

    pub fn set_max_members(&mut self, max_members: usize) {
        self.max_members = max_members;
    }

    pub fn get_num_of_members(self) -> usize {
        self.members.len()
    }

    pub fn add_member(&mut self, id: usize) {
        if self.members.len() >= self.max_members {
            panic!("max members {} exceeded", self.max_members);
        }
        self.members.push(id);
    }

    pub fn remove_member(&mut self, id: &usize) {
        self.members.retain(|s| s != id);
    }
}
