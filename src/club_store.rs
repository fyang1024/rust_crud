use crate::club;

pub struct ClubStore(Vec<club::Club>);

impl ClubStore {
    pub fn new() -> Self {
        Self(Vec::<club::Club>::new())
    }

    pub fn create_club(
        &mut self,
        name: String,
        description: String,
        max_members: usize,
    ) -> &club::Club {
        let club = club::Club::new(self.0.len(), name, description, max_members);
        self.0.push(club);
        &self.0.last().unwrap()
    }

    pub fn find_club(&mut self, name: &String) -> Option<&club::Club> {
        self.0.iter().find(|c| c.get_name() == name)
    }

    pub fn find_club_mut(&mut self, name: &String) -> Option<&mut club::Club> {
        self.0.iter_mut().find(|c| c.get_name() == name)
    }

    pub fn update_club(&mut self, name: &String, new_description: String, new_max_members: usize) {
        let club = self.find_club_mut(name);
        match club {
            Some(c) => {
                c.set_description(new_description);
                c.set_max_members(new_max_members);
            }
            None => println!("No club found with name {}", name),
        }
    }

    pub fn update_club_description(&mut self, name: &String, new_description: String) {
        let club = self.find_club_mut(name);
        match club {
            Some(c) => c.set_description(new_description),
            None => println!("No club found with name {}", name),
        }
    }

    pub fn update_club_max_members(&mut self, name: &String, new_max_members: usize) {
        let club = self.find_club_mut(name);
        match club {
            Some(c) => c.set_max_members(new_max_members),
            None => println!("No club found with name {}", name),
        }
    }

    pub fn delete_club(&mut self, club: &club::Club) {
        self.0.retain(|c| &c != &club)
    }
}
