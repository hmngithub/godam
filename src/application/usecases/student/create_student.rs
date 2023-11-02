use crate::{
    application::{
        dto::CreateStudentReq,
        gateways::{IdGenerator, StudentRepository},
    },
    domain::student::entity::Student,
};

use super::Usecases;

pub struct CreateStudent {
    repo: Box<dyn StudentRepository>,
    id_gen: Box<dyn IdGenerator>,
}

impl CreateStudent {
    pub fn new(repo: Box<dyn StudentRepository>, id_gen: Box<dyn IdGenerator>) -> Self {
        Self { repo, id_gen }
    }
}

impl Usecases<CreateStudentReq, String> for CreateStudent {
    fn execute(&mut self, request: CreateStudentReq) -> String {
        let id = self.id_gen.generate_id();
        let new_student = Student::new()
            .set_id(id)
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

        self.repo.create_student(new_student);
        "user add succesfully".to_string()
    }
}
