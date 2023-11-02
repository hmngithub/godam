use crate::domain::student::enums::Major;

pub struct Student {
    id: Option<String>,
    name: Option<String>,
    last_name: Option<String>,
    father_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    major: Option<Major>,
    semester_count: Option<u8>,
    fece: Option<u16>,
    recive: Option<u16>,
    is_available: bool,
}

impl Student {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            last_name: None,
            father_name: None,
            email: None,
            phone_number: None,
            major: None,
            semester_count: None,
            fece: None,
            recive: None,
            is_available: true,
        }
    }
}

impl Student {
    pub fn set_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn set_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }
    pub fn set_father_name(mut self, father_name: impl Into<String>) -> Self {
        self.father_name = Some(father_name.into());
        self
    }
    pub fn set_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
    pub fn set_phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.phone_number = Some(phone_number.into());
        self
    }
    pub fn set_major(mut self, major: Major) -> Self {
        self.major = Some(major);
        self
    }
    pub fn set_semester_count(mut self, semester_count: u8) -> Self {
        self.semester_count = Some(semester_count);
        self
    }
    pub fn set_fece(mut self, fece: u16) -> Self {
        self.fece = Some(fece);
        self
    }
    pub fn set_recive(mut self, recive: u16) -> Self {
        self.recive = Some(recive);
        self
    }
    pub fn set_is_available(mut self, is_available: bool) -> Self {
        self.is_available = is_available;
        self
    }
}
