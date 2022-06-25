#![allow(non_snake_case)]
mod Student;
mod Sorter;
use std::{env, fs};
use serde_json::{Value, Map};
use std::error::Error;
use std::env::current_exe;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()!=1{
        //do stuff with setting and getting things
    }
    else{

    }
    test();
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

fn read_config(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}