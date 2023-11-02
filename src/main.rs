use godam::{
    application::gateways::StudentRepository,
    infrastructure::{IDGeneratorImp, StudentRepoInFile, SubjectRepoInFile, SubjectRepoInMem},
    presentation::cli::ArgControll,
};

fn main() {
    let student_repo = Box::new(StudentRepoInFile::new());
    let subject_repo = Box::new(SubjectRepoInFile::new());
    let id_gen = Box::new(IDGeneratorImp::new());
    ArgControll::new(student_repo, subject_repo, id_gen).execute();
}
