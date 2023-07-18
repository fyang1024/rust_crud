use crate::grade;

#[derive(Debug)]
pub struct Lesson {
    name: String,
    description: String,
    grades: Vec<grade::Grade>,
}

impl Lesson {
    pub fn new(name: String, description: String, grades: Vec<grade::Grade>) -> Lesson {
        Lesson {
            name,
            description,
            grades,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn get_grades(&self) -> &Vec<grade::Grade> {
        &self.grades
    }

    pub fn set_grades(&mut self, new_grades: Vec<grade::Grade>) {
        self.grades = new_grades
    }
}
