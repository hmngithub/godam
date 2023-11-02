mod create_student;
mod delete_student;
mod fetch_student;
mod fetchs_students;
mod update_student;
pub use create_student::CreateStudent;
pub use delete_student::DeleteStudent;
pub use fetch_student::FetchStudent;
pub use fetchs_students::FetchsStudents;
pub use update_student::UpdateStudent;
pub trait Usecases<Request, Response> {
    fn execute(&mut self, request: Request) -> Response;
}
