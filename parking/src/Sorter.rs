#![allow(non_snake_case)]
use std::collections::VecDeque;
use std::fs;
use std::io::Error;
use serde_json::{Value, Map};

use crate::Student;

pub struct Sorter{
    pub students: Vec<Vec<Student::Student>>,
    pub student_names: Vec<String>,
    MAX_REG: usize,
    MAX_SML: usize,
    MAX_PAR: usize,
}

impl Sorter{
    pub fn new(student_data: Vec<Vec<Student::Student>>, config_path: &str) -> Result<Sorter, Error>{
        let mut names: Vec<String> = Vec::new();
        for s in &student_data[0]{names.push(s.get_name())}
        names.sort();

        let config = fs::read_to_string(config_path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let cfg: Map<String, Value> = parsed.as_object().unwrap().clone();



        let mut s = Sorter {
            students: student_data, //should be [monday[student, student, ...], tuesday[student, student, student]]
            student_names: names,
            MAX_REG: cfg["MAX_REG"].to_string().parse::<usize>().unwrap(),
            MAX_SML: cfg["MAX_SML"].to_string().parse::<usize>().unwrap(),
            MAX_PAR: cfg["MAX_PAR"].to_string().parse::<usize>().unwrap(),

        };
        s.sort_days();
        
        Ok(s)
    }

    fn sort_days(&mut self){
        let mut day_num = 0;
        for day in 0..self.students.len(){
            self.students[day].sort_by(|a, b| a.generate_score(day_num).partial_cmp(&b.generate_score(day_num)).unwrap());
            day_num += 1;
        }
    }

    fn index(&self, student: String) -> Result<usize, String>{
        //! SLOW! but I don't have wifi to search up the builtin version ;-;
        let mut idx: usize = 0;
        for name in &self.student_names{
            if name == &student{
                return Ok(idx);
            } 
            idx += 1;
        }
        Err(String::from("Student Not Found"))
    }

    pub fn get_assignments(&self) -> Vec<Vec<String>>{
        //should output [name, mon, tues, wed, thur, fri] for each student
        //[name    mon    tue    wed    thu    fri]
        //                ...
        //[abcd    brt    reg    sml    sml    brt]

        //*so start by doing this, then transpose
        //*this way we can go day by day
        //[stu1    stu2    stu3    stu4    ....    stuN]
        //[zMon    zMon    zMon    zMon    ....    zMon]
        //[zTue    zTue    zTue    zTue    ....    zTue]
        
        let mut results: Vec<Vec<String>> = Vec::new();
        
        for day in &self.students{
            let mut this_day_results: Vec<String> = vec![String::new(); day.len()];
            let mut double_ability_students = VecDeque::new();
            let mut par_tot: usize = 0;
            let mut sml_tot: usize = 0;
            let mut reg_tot: usize = 0;
            for student in day{
                let student_idx = self.index(student.get_name()).unwrap();
                
                //catching the corner case! from the python code:
                //>  # problem: if someone can park in par and small, they get put in par, but if there's someone 
                //>  # else later who can park in par but not sml, and par and reg is full, but sml is not, they'd 
                //>  # get put in barts even when they can park on campus
                if student.can_parallel_park() && student.has_small_car(){
                    double_ability_students.push_back(student);
                }
                if {student.can_parallel_park()&& //if the student can parallel park
                    reg_tot >= self.MAX_REG && //but reg
                    par_tot >= self.MAX_PAR && //and par are full
                    sml_tot <  self.MAX_SML && // and sml isn't
                    double_ability_students.len() != 0}{ //and there's someone who could be moved from par to sml to allow this person to park
                    this_day_results[self.index(double_ability_students.pop_front().unwrap().get_name()).unwrap()] = String::from("SML");
                    this_day_results[student_idx] = String::from("PAR");
                    sml_tot += 1;

                }
                //ok that should be taken care of now, we can sort normally

                else if student.can_parallel_park() && par_tot < self.MAX_PAR{
                    this_day_results[student_idx] = String::from("PAR");
                    par_tot += 1;
                }
                else if student.has_small_car() && sml_tot < self.MAX_SML{
                    this_day_results[student_idx] = String::from("SML");
                    sml_tot += 1;

                }
                else if reg_tot < self.MAX_REG{
                    this_day_results[student_idx] = String::from("REG");
                    reg_tot += 1;

                }
                else{
                    this_day_results[student_idx] = String::from("BART");

                }
            }

            results.push(this_day_results);
        }
        //transpose and return!
        let mut output: Vec<Vec<String>> = Vec::new();
        for s in 0..self.students[0].len(){
            let mut col: Vec<String> = vec![self.student_names[s].clone()];
            for row in 0..5{ //there are 6 rows: name + 5 days
                col.push(results[row][s].clone());
            }
            output.push(col);//now col is a row, so transposed
        }
        output

    }
}