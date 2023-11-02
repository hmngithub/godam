use crate::application::gateways::SubjectRepository;

pub struct SubjectRepoInFile {}

impl SubjectRepoInFile {
    pub fn new() -> Self {
        Self {}
    }
}

impl SubjectRepository for SubjectRepoInFile {
    fn create_subject(&mut self, subject: crate::domain::subject::entity::Subject) {
        todo!()
    }

    fn delete_subject(&mut self, id: String) {
        todo!()
    }

    fn update_subject(&mut self, new_subject: crate::domain::subject::entity::Subject) {
        todo!()
    }

    fn fetch_subject(
        &mut self,
        query: crate::application::dto::FetchSubjectReq,
    ) -> crate::domain::subject::entity::Subject {
        todo!()
    }

    fn fetchs_subjects(&mut self, q: String) -> Vec<crate::domain::subject::entity::Subject> {
        todo!()
    }

    fn fetchs_subjects_by_parent_id(
        &mut self,
        parent_id: String,
    ) -> Vec<crate::domain::subject::entity::Subject> {
        todo!()
    }
}
