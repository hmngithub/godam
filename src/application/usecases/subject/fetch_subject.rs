use crate::{
    application::{dto::FetchSubjectReq, gateways::SubjectRepository},
    domain::subject::entity::Subject,
};

use super::Usecases;

pub struct FetchSubject {
    repo: Box<dyn SubjectRepository>,
}

impl FetchSubject {
    pub fn new(repo: Box<dyn SubjectRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<FetchSubjectReq, Subject> for FetchSubject {
    fn execute(&mut self, request: FetchSubjectReq) -> Subject {
        self.repo.fetch_subject(request)
    }
}
