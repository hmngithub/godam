use crate::domain::student::entity::Student;

pub trait StudentRepository {
    fn create_student(&mut self, student: Student);
    fn delete_student(&mut self, id: String);
    fn update_student(&mut self, new_student: Student);
    fn fetch_student(&mut self, q: String) -> Student;
    fn fetchs_student(&mut self, q: String) -> Vec<Student>;
}
