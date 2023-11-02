use std::cell::RefCell;

use clap::{Arg, Command};

use crate::application::gateways::{IdGenerator, StudentRepository, SubjectRepository};

use super::Run;

pub struct ArgControll {
    student_repo: Box<dyn StudentRepository>,
    subject_repo: Box<dyn SubjectRepository>,
    id_gen: Box<dyn IdGenerator>,
}

impl ArgControll {
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

impl ArgControll {
    pub fn execute(self) {
        let name = Arg::new("001").short('n').long("name").required(true);

        let subject = Command::new("subject").arg(name);
        let student = Command::new("student");

        let create_sub = Command::new("create")
            .override_usage("godam create [COMMAND]")
            .about("you can create [subject,student]")
            .subcommand(subject.clone())
            .subcommand(student.clone())
            .arg_required_else_help(true)
            .subcommand_required(true);

        let delete_sub = Command::new("delete")
            .about("you can delete [subject,student]")
            .subcommand(subject.clone())
            .subcommand(student.clone())
            .arg_required_else_help(true);
        let update_sub = Command::new("update")
            .about("you can update [subject,student]")
            .subcommand(subject.clone())
            .subcommand(student.clone())
            .arg_required_else_help(true);
        let show_sub = Command::new("show")
            .about("you can show [subject,student]")
            .subcommand(subject.clone())
            .subcommand(student.clone())
            .arg_required_else_help(true);
        let filter_sub = Command::new("filter")
            .about("you can filter showing [subject,student]")
            .subcommand(subject.clone())
            .subcommand(student.clone())
            .arg_required_else_help(true);

        let run_sub = Command::new("run").about("you can run goda cli app in a loop");

        let command = Command::new("godam")
            .override_usage("godam [COMMAND] OR godam [OPTION]")
            .version("v1.0.1")
            .about("Godam is a cli app database for university that mannege students data ")
            .subcommand(create_sub)
            .subcommand(delete_sub)
            .subcommand(update_sub)
            .subcommand(show_sub)
            .subcommand(filter_sub)
            .subcommand(run_sub)
            .author("Homayoun mohammadi")
            .arg_required_else_help(true)
            .disable_help_subcommand(true)
            .get_matches();

        if command.subcommand_matches("run").is_some() {
            let run = Run::new(self.student_repo, self.subject_repo, self.id_gen);
            run.exicute();
            // let mut buffer = String::new();
            // print!("enter your command : ");
            // std::io::stdout().flush().unwrap();
            // std::io::stdin().read_line(&mut buffer).unwrap();
            // let repo = Box::new(StudentRepoInFile::new());
            // let id_gen = Box::new(IDGeneratorImp::new());
            // let create_usecase = CreateStudent::new(repo, id_gen);
        } else if command.subcommand_matches("create").is_some() {
            let d = command.subcommand_matches("create").expect("sdfs");
            if d.subcommand_matches("subject").is_some() {
                let m = d.subcommand_matches("subject").expect("dd");
                let value = m.get_one::<String>("001").expect("ddd");
                println!("{}", value);
            }
        }
    }
}
