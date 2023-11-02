mod create_subject;
mod delete_subject;
mod fetch_subject;
mod fetchs_subjects;
mod fetchs_subjects_by_parent_id;
mod update_subject;
pub use create_subject::CreateSubject;
pub use delete_subject::DeleteSubject;
pub use fetch_subject::FetchSubject;
pub use fetchs_subjects::FetchsSubjects;
pub use fetchs_subjects_by_parent_id::FetchsSubjectsByParentId;
pub use update_subject::UpdateSubject;

pub trait Usecases<Request, Response> {
    fn execute(&mut self, request: Request) -> Response;
}
