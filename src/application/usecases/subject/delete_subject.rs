use crate::application::gateways::SubjectRepository;

use super::Usecases;

pub struct DeleteSubject {
    repo: Box<dyn SubjectRepository>,
}

impl DeleteSubject {
    pub fn new(repo: Box<dyn SubjectRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<String, String> for DeleteSubject {
    fn execute(&mut self, request: String) -> String {
        self.repo.delete_subject(request);
        "subject deleted succesfully".to_string()
    }
}
