use crate::{application::gateways::StudentRepository, domain::student::entity::Student};

use super::Usecases;

pub struct FetchsStudents {
    repo: Box<dyn StudentRepository>,
}

impl FetchsStudents {
    pub fn new(repo: Box<dyn StudentRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<String, Vec<Student>> for FetchsStudents {
    fn execute(&mut self, request: String) -> Vec<Student> {
        self.repo.fetchs_student(request)
    }
}
