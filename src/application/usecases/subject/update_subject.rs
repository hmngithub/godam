use crate::{
    application::{dto::UpdateSubjectReq, gateways::SubjectRepository},
    domain::subject::entity::Subject,
};

use super::Usecases;

pub struct UpdateSubject {
    repo: Box<dyn SubjectRepository>,
}

impl UpdateSubject {
    pub fn new(repo: Box<dyn SubjectRepository>) -> Self {
        Self { repo }
    }
}

impl Usecases<UpdateSubjectReq, String> for UpdateSubject {
    fn execute(&mut self, request: UpdateSubjectReq) -> String {
        let new_subject = Subject::new()
            .set_id(request.id.unwrap())
            .set_parent_id(request.parent_id.unwrap())
            .set_name(request.name.unwrap())
            .set_value_20(request.value_20.unwrap())
            .set_value_60(request.value_60.unwrap());

        self.repo.update_subject(new_subject);
        "subject updated succesfully".to_string()
    }
}
