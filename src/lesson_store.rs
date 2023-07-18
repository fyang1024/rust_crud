use std::collections::HashMap;

use crate::grade;
use crate::lesson;

pub struct LessonStore(HashMap<String, lesson::Lesson>);

impl LessonStore {
    pub fn new() -> Self {
        Self(HashMap::<String, lesson::Lesson>::new())
    }

    pub fn create_lesson(
        &mut self,
        name: String,
        description: String,
        grades: Vec<grade::Grade>,
    ) -> &lesson::Lesson {
        let lesson = lesson::Lesson::new(name.clone(), description, grades);
        self.0.insert(name.clone(), lesson);
        self.0.get(&name).unwrap()
    }

    pub fn find_lesson(&mut self, name: &String) -> Option<&lesson::Lesson> {
        self.0.get(name)
    }

    pub fn find_lesson_mut(&mut self, name: &String) -> Option<&mut lesson::Lesson> {
        self.0.get_mut(name)
    }

    pub fn update_lesson(&mut self, name: &String, description: String, grades: Vec<grade::Grade>) {
        let lesson = self.find_lesson_mut(name);
        match lesson {
            Some(l) => {
                l.set_description(description);
                l.set_grades(grades);
            }
            None => println!("No lesson found for name {}", name),
        }
    }

    pub fn update_lesson_description(&mut self, name: &String, description: String) {
        let lesson = self.find_lesson_mut(name);
        match lesson {
            Some(l) => l.set_description(description),
            None => println!("No lesson found for name {}", name),
        }
    }

    pub fn update_lesson_grades(&mut self, name: &String, grades: Vec<grade::Grade>) {
        let lesson = self.find_lesson_mut(name);
        match lesson {
            Some(l) => l.set_grades(grades),
            None => println!("No lesson found for name {}", name),
        }
    }

    pub fn delete_lesson(&mut self, lesson: &lesson::Lesson) {
        self.0.remove(lesson.get_name());
    }
}
