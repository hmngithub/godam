use crate::domain::student::enums::Major;

pub struct CreateStudentReq {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub father_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub major: Option<Major>,
    pub semester_count: Option<u8>,
    pub fece: Option<u16>,
    pub recive: Option<u16>,
    pub is_available: bool,
}
