use crate::application::gateways::StudentRepository;

pub struct StudentRepoInFile {}

impl StudentRepoInFile {
    pub fn new() -> Self {
        Self {}
    }
}

impl StudentRepository for StudentRepoInFile {
    fn create_student(&mut self, student: crate::domain::student::entity::Student) {}

    fn delete_student(&mut self, id: String) {
        todo!()
    }

    fn update_student(&mut self, new_student: crate::domain::student::entity::Student) {
        todo!()
    }

    fn fetch_student(&mut self, q: String) -> crate::domain::student::entity::Student {
        todo!()
    }

    fn fetchs_student(&mut self, q: String) -> Vec<crate::domain::student::entity::Student> {
        todo!()
    }
}
