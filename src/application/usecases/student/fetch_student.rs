use crate::{application::gateways::StudentRepository, domain::student::entity::Student};

use super::Usecases;

pub struct FetchStudent {
    repo: Box<dyn StudentRepository>,
}

impl FetchStudent {
    pub fn new(repo: Box<dyn StudentRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<String, Student> for FetchStudent {
    fn execute(&mut self, request: String) -> Student {
        let new_student = self.repo.fetch_student(request);
        new_student
    }
}
