use crate::{application::gateways::SubjectRepository, domain::subject::entity::Subject};

use super::Usecases;

pub struct FetchsSubjectsByParentId {
    repo: Box<dyn SubjectRepository>,
}

impl FetchsSubjectsByParentId {
    pub fn new(repo: Box<dyn SubjectRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<String, Vec<Subject>> for FetchsSubjectsByParentId {
    fn execute(&mut self, request: String) -> Vec<Subject> {
        self.repo.fetchs_subjects_by_parent_id(request)
    }
}
