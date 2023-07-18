use std::collections::HashMap;

use crate::class;
use crate::student;

pub struct StudentStore(HashMap<usize, student::Student>);

impl StudentStore {
    pub fn new() -> Self {
        Self(HashMap::<usize, student::Student>::new())
    }

    pub fn create_student(
        &mut self,
        full_name: String,
        class: &mut class::Class,
    ) -> &student::Student {
        let id = self.0.len();
        let student = student::Student::new(id, full_name, *class.get_grade(), *class.get_class());
        self.0.insert(id, student);
        class.add_student(id);
        self.0.get(&id).unwrap()
    }

    pub fn find_student(&self, id: &usize) -> Option<&student::Student> {
        self.0.get(id)
    }

    pub fn find_student_mut(&mut self, id: &usize) -> Option<&mut student::Student> {
        self.0.get_mut(id)
    }

    pub fn update_student_full_name(&mut self, id: &usize, full_name: String) {
        let student = self.find_student_mut(id);
        match student {
            Some(s) => s.set_full_name(full_name),
            None => println!("No student found for student number {}", id),
        }
    }

    pub fn delete_student(&mut self, student_id: usize) {
        self.0.remove(&student_id);
    }
}
