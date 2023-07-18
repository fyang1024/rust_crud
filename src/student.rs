use crate::club;
use crate::grade;

#[derive(PartialEq, Eq, Debug)]
pub struct Student {
    id: usize,
    full_name: String,
    grade: grade::Grade,
    class: u8,
    clubs: Vec<usize>,
}

impl Student {
    pub fn new(id: usize, full_name: String, grade: grade::Grade, class: u8) -> Student {
        Student {
            id,
            full_name,
            grade,
            class,
            clubs: Vec::<usize>::new(),
        }
    }

    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_full_name(&self) -> &String {
        &self.full_name
    }

    pub fn set_full_name(&mut self, new_full_name: String) {
        self.full_name = new_full_name;
    }

    pub fn get_grade(&self) -> grade::Grade {
        self.grade
    }

    pub fn get_class(&self) -> u8 {
        self.class
    }

    pub fn set_grade_class(&mut self, grade: grade::Grade, class: u8) {
        self.grade = grade;
        self.class = class;
    }

    pub fn join_club(&mut self, club: &mut club::Club) {
        self.clubs.push(club.get_id());
        club.add_member(self.id);
    }

    pub fn leave_club(&mut self, club: &mut club::Club) {
        self.clubs.retain(|id| id != &club.get_id());
        club.remove_member(&self.id);
    }
}
