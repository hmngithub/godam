use crate::application::gateways::StudentRepository;

use super::Usecases;

pub struct DeleteStudent {
    repo: Box<dyn StudentRepository>,
}

impl DeleteStudent {
    pub fn new(repo: Box<dyn StudentRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<String, String> for DeleteStudent {
    fn execute(&mut self, request: String) -> String {
        self.repo.delete_student(request.clone());
        return format!("student in this [{}] id deleted succesfully", request);
    }
}
