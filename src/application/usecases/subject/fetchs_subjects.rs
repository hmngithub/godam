use crate::{application::gateways::SubjectRepository, domain::subject::entity::Subject};

use super::Usecases;

pub struct FetchsSubjects {
    repo: Box<dyn SubjectRepository>,
}

impl FetchsSubjects {
    pub fn new(repo: Box<dyn SubjectRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<String, Vec<Subject>> for FetchsSubjects {
    fn execute(&mut self, request: String) -> Vec<Subject> {
        self.repo.fetchs_subjects(request)
    }
}
