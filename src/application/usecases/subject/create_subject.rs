use crate::{
    application::{
        dto::CreateSubjectReq,
        gateways::{IdGenerator, SubjectRepository},
    },
    domain::subject::entity::Subject,
};

use super::Usecases;

pub struct CreateSubject {
    repo: Box<dyn SubjectRepository>,
    id_gen: Box<dyn IdGenerator>,
}

impl CreateSubject {
    pub fn new(repo: Box<dyn SubjectRepository>, id_gen: Box<dyn IdGenerator>) -> Self {
        Self { repo, id_gen }
    }
}

impl Usecases<CreateSubjectReq, String> for CreateSubject {
    fn execute(&mut self, request: CreateSubjectReq) -> String {
        let id = self.id_gen.generate_id();
        let new_subject = Subject::new()
            .set_id(id)
            .set_parent_id(request.parent_id.unwrap())
            .set_name(request.name.unwrap())
            .set_value_20(request.value_20.unwrap())
            .set_value_60(request.value_60.unwrap());
        self.repo.create_subject(new_subject);
        "subject create succesfully".to_string()
    }
}
