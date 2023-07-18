use crate::grade;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Class {
    grade: grade::Grade,
    class: u8,
    students: Vec<usize>,
}

impl Class {
    pub fn new(grade: grade::Grade, class: u8) -> Class {
        Self {
            grade,
            class,
            students: Vec::<usize>::new(),
        }
    }

    pub fn get_grade(&self) -> &grade::Grade {
        &self.grade
    }

    pub fn get_class(&self) -> &u8 {
        &self.class
    }

    pub fn add_student(&mut self, id: usize) {
        self.students.push(id);
    }

    pub fn remove_student(&mut self, id: &usize) {
        self.students.retain(|s| s != id);
    }
}
