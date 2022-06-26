#![allow(non_snake_case)]
mod Student;
mod Sorter;
use std::collections::btree_set::Intersection;
use std::ops::Index;
use std::{env, fs};
use serde_json::{Value, Map};
use std::io::Error;
use std::env::current_exe;

fn main() {
    const VERSION: &str = "0.0.1";
    let mut args: Vec<String> = env::args().collect();
    if args.len()!=1{
        if args[1] == "config" {
            if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()){
                config_help();
                return;
            }
            if args.contains(&"-s".to_string()) || args.contains(&"--show".to_string()){
                
            }
        }
        if args[1] == "assign" {
            if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()){
                assign_help();
                return;
            }
            if args.contains(&"-t".to_string()) || args.contains(&"--time".to_string()){
                let time = args.pop().unwrap();
                println!("assignment scheduled for {}", time);
                //todo: wait the time
            }
            run_sort();
            return;
        }
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()){
            main_help();
            return;
        }
        if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()){
            println!("Parking v{}", VERSION);
            return;
        }
    }
    else{

    }
    test();
}

fn run_sort(){} //TODO: fill this out

fn assign_help(){
    println!("usage: parking assign [-h] [-t TIME]\n");
    println!("run the algorithm and assign parking for the week.\n");
    println!("OPTIONS:");
    println!("  -h, --help           show this help message and exit");
    println!("  -t, --time [TIME]    assign parking at TIME today, where TIME is a 24 hour time given in HH:MM format (ie, -t 24:00 would mean midnight)");
}
fn config_help(){
    println!("usage: parking config [-h] [-s] [-e] SETTING NEW_VALUE\n");
    println!("change or view the current configuration.\n");
    println!("POSITIONAL ARGUMENTS:");
    println!("  SETTING         (optional) the setting to change.");
    println!("  NEW_VALUE       (optional) the setting's new value. If given, SETTING must also be given.\n");
    println!("OPTIONS:");
    println!("  -h, --help      show this help message and exit");
    println!("  -s, --show      show the given setting's value. If no setting is given, list all settings and their values.");
    println!("  -e, --editor    open an editor with the setting to edit the value.")

}

fn main_help(){
    println!("usage: parking [-h] [-v] <command> [<args>]\n");
    println!("an app to assign on-campus parking spots.\n");
    println!("OPTIONS:");
    println!("   -h, --help      show this help message and exit");
    println!("   -v, --version   print the current version and exit\n");
    println!("COMMANDS:\n");
    println!("  config           view or change the configuration of max sizes, weights, or paths.");
    println!("  assign           runs the algorithm and assigns parking for the week\n");
    println!("for help relating to config or assign, pass the -h flag: `~$ parking config -h`");
}
fn test(){
    let path_buffer = current_exe().unwrap();
    let path_temp = path_buffer.to_str().unwrap();
    let mut path_vec: Vec<&str> = path_temp.split('/').collect();
    path_vec.pop();
    let mut path = path_vec.join("/");
    path.push_str("/.config.json");

    let s = Sorter::Sorter::new(vec![
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap()],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap()],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap()],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap()],
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("joe", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("frank", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap(),
             Student::Student::new("kevin", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], &path).unwrap()],
    ], &path).unwrap();
    let assignments = s.get_assignments();
    for s in assignments{
        println!("{:?}", s);
    }

}

fn read_config(path: &str) -> Result<Map<String, Value>, Error> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}