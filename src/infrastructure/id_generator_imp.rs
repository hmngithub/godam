use rand::Rng;

use crate::application::gateways::IdGenerator;

pub struct IDGeneratorImp;

impl IDGeneratorImp {
    pub fn new() -> Self {
        Self
    }
}
impl IdGenerator for IDGeneratorImp {
    fn generate_id(&mut self) -> String {
        let mut buffer = String::new();
        buffer.push_str("TODO");
        let id = rand::thread_rng().gen_range(100000..=100000000);
        buffer.push_str(&format!("{id}"));
        buffer
    }
}
