use crate::class;
use crate::grade;
pub struct ClassStore(Vec<class::Class>);

impl ClassStore {
    pub fn new() -> Self {
        Self(Vec::<class::Class>::new())
    }

    pub fn create_class(&mut self, grade: grade::Grade, class: u8) -> &mut class::Class {
        let c = class::Class::new(grade, class);
        self.0.push(c);
        self.0.last_mut().unwrap()
    }

    pub fn find_class(&self, grade: &grade::Grade, class: &u8) -> Option<&class::Class> {
        self.0
            .iter()
            .find(|c| c.get_grade() == grade && c.get_class() == class)
    }

    pub fn find_class_mut(
        &mut self,
        grade: &grade::Grade,
        class: &u8,
    ) -> Option<&mut class::Class> {
        self.0
            .iter_mut()
            .find(|c| c.get_grade() == grade && c.get_class() == class)
    }

    pub fn delete_class(&mut self, class: &class::Class) {
        self.0.retain(|c| &c != &class)
    }
}
