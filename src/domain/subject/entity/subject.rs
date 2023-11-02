pub struct Subject {
    id: Option<String>,
    parent_id: Option<String>,
    name: Option<String>,
    value_20: Option<u16>,
    value_60: Option<u16>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            id: None,
            parent_id: None,
            name: None,
            value_20: None,
            value_60: None,
        }
    }
}

impl Subject {
    pub fn set_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn set_parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }
    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn set_value_20(mut self, value_20: impl Into<u16>) -> Self {
        self.value_20 = Some(value_20.into());
        self
    }
    pub fn set_value_60(mut self, value_60: impl Into<u16>) -> Self {
        self.value_60 = Some(value_60.into());
        self
    }
}

impl Subject {
    pub fn read_value_20(&self) -> String {
        if self.value_20.is_some() {
            return self.value_20.unwrap().to_string();
        } else {
            return "value_20 is empty".to_string();
        }
    }
    pub fn read_value_60(&self) -> String {
        if self.value_60.is_some() {
            return self.value_60.unwrap().to_string();
        } else {
            return "value_60 is empty".to_string();
        }
    }
    pub fn read_value_100(&self) -> String {
        if self.value_60.is_some() && self.value_20.is_some() {
            return (self.value_60.unwrap() + self.value_20.unwrap()).to_string();
        } else if self.value_60.is_some() {
            return self.value_60.unwrap().to_string();
        } else if self.value_20.is_some() {
            return self.value_20.unwrap().to_string();
        } else {
            return "value_20 is empty".to_string();
        }
    }
}
