mod Student;
mod Sorter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cur_arg = 1;
    for arg in &args[1..]{
        if arg.starts_with("--"){
            let stripped = &arg[2..];
            if stripped == "set-max"{
                
            }
            else if stripped == "load-students"{

            }
            else if stripped == "list-config"{
                
            }

        }
        else if arg.starts_with("-") && arg.len()>1{
            for flag in String::from(arg).chars().into_iter(){

            }
        }

        cur_arg += 1;
    }
    test();
}
fn test(){
    let s = Sorter::Sorter::new(vec![
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
    ]);
    let assignments = s.get_assignments();
    for s in assignments{
        println!("{:?}", s);
    }

}
