use std::io::Write;

use crate::application::gateways::{IdGenerator, StudentRepository, SubjectRepository};

pub struct Run {
    student_repo: Box<dyn StudentRepository>,
    subject_repo: Box<dyn SubjectRepository>,
    id_gen: Box<dyn IdGenerator>,
}

impl Run {
    pub fn new(
        student_repo: Box<dyn StudentRepository>,
        subject_repo: Box<dyn SubjectRepository>,
        id_gen: Box<dyn IdGenerator>,
    ) -> Self {
        Self {
            student_repo,
            subject_repo,
            id_gen,
        }
    }
}

impl Run {
    pub fn exicute(&self) {
        self.print_welcom();
        loop {
            let mut buffer = String::new();
            print!("enter your command : ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();
        }
    }

    fn print_welcom(&self) {
        println!("---------------------------------------------------------------");
        println!("           WELLCOM TO GODAM STUDENT DATABASE MANIGMENT         ");
        println!("---------------------------------------------------------------");
    }
}
