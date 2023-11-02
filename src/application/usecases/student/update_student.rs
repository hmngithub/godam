use crate::{
    application::{dto::UpdateStudentReq, gateways::StudentRepository},
    domain::student::entity::Student,
};

use super::Usecases;

pub struct UpdateStudent {
    repo: Box<dyn StudentRepository>,
}

impl UpdateStudent {
    pub fn new(repo: Box<dyn StudentRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<UpdateStudentReq, String> for UpdateStudent {
    fn execute(&mut self, request: UpdateStudentReq) -> String {
        let new_student = Student::new()
            .set_id(request.id.unwrap())
            .set_name(request.name.unwrap())
            .set_last_name(request.last_name.unwrap())
            .set_father_name(request.father_name.unwrap())
            .set_major(request.major.unwrap())
            .set_email(request.email.unwrap())
            .set_phone_number(request.phone_number.unwrap())
            .set_fece(request.fece.unwrap())
            .set_recive(request.recive.unwrap())
            .set_is_available(true)
            .set_semester_count(request.semester_count.unwrap());

        self.repo.update_student(new_student);
        let massege = format!("student succesfully updated",);
        massege
    }
}
