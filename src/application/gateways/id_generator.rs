pub trait IdGenerator {
    fn generate_id(&mut self) -> String;
}
