use crate::{application::dto::FetchSubjectReq, domain::subject::entity::Subject};

pub trait SubjectRepository {
    fn create_subject(&mut self, subject: Subject);
    fn delete_subject(&mut self, id: String);
    fn update_subject(&mut self, new_subject: Subject);
    fn fetch_subject(&mut self, query: FetchSubjectReq) -> Subject;
    fn fetchs_subjects(&mut self, q: String) -> Vec<Subject>;
    fn fetchs_subjects_by_parent_id(&mut self, parent_id: String) -> Vec<Subject>;
}
