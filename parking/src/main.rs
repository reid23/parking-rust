//mod Sorter;
use std::fs;
use std::error::Error;
use serde_json::{Value, Map};
use rand_distr::{Normal, Distribution};

pub struct Student{
    name: String,
    row: Vec<u8>,
    dists: Map<String, Value>
}

impl Student{
    fn read_config(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }

    pub fn new(student_name: &str, row: &Vec<u8>) -> Student{
        Student {
            name: String::from(student_name),
            row: row.clone(),
            dists: Student::read_config("/Users/reiddye/parking-rust/distances.json").unwrap()
        }
    }

    pub fn generate_score(&self, day: usize) -> f32{
        let crit: u8 = self.row[2..7][day];
        let sports: u8 = self.row[7..12][day];
        let fp_free = self.row[14];
        let lp_free = self.row[15];
        let dist: f32 = self.dists.get(&self.row[1].to_string()).unwrap().as_f64().unwrap() as f32;
        let carpool_seniors: u8 = self.row[12];
        let carpool_youngns: u8 = self.row[13];
        let strikes = self.row[17];
        let weights: Vec<f32> = vec![16.0, 8.0, 10.0, 40.0, -20.0];
        let mut score: f32 = weights.iter().zip(vec![fp_free, lp_free, crit, sports, strikes].iter()).map(|(x, y)| x * (*y as f32)).sum();
        score *= 1.0 + carpool_seniors as f32 + (carpool_youngns as f32 * 0.25);
        
        let normal = Normal::new(0.0 as f32, 5.0 as f32).unwrap();
        score += normal.sample(&mut rand::thread_rng());
        score += dist;

        score
    }

    pub fn can_parallel_park(&self) -> bool{
        self.row[16] == 1
    }
    pub fn has_small_car(&self) -> bool{
        self.row[0] == 1
    }
    pub fn get_name(&self) -> String{
        String::from(&self.name)
    }

}

use std::collections::VecDeque;

pub struct Sorter{
    students: Vec<Vec<Student>>,
    student_names: Vec<String>
}

impl Sorter{
    const MAX_REG: usize = 41;
    const MAX_SML: usize = 8;
    const MAX_PAR: usize = 4;
    
    pub fn new(all_students: Vec<Vec<Student>>) -> Sorter{
        let mut names: Vec<String> = Vec::new();
        for s in &all_students[0]{names.push(s.get_name())}
        names.sort();

        let mut s = Sorter {
            students: all_students, //should be [monday[student, student, ...], tuesday[student, student, student]]
            student_names: names
        };
        s.sort_days();
        
        s
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
        let mut par_tot: usize = 0;
        let mut sml_tot: usize = 0;
        let mut reg_tot: usize = 0;
        let mut double_ability_students = VecDeque::new();

        for day in &self.students{
            let mut this_day_results: Vec<String> = vec![String::new(); day.len()];
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
                    reg_tot >= Sorter::MAX_REG && //but reg
                    par_tot >= Sorter::MAX_PAR && //and par are full
                    sml_tot <  Sorter::MAX_SML && // and sml isn't
                    double_ability_students.len() != 0}{ //and there's someone who could be moved from par to sml to allow this person to park
                    this_day_results[self.index(double_ability_students.pop_front().unwrap().get_name()).unwrap()] = String::from("SML");
                    this_day_results[student_idx] = String::from("PAR");
                    sml_tot += 1;
                }
                //ok that should be taken care of now, we can sort normally

                else if student.can_parallel_park() && par_tot < Sorter::MAX_PAR{
                    this_day_results[student_idx] = String::from("PAR");
                    par_tot += 1;
                }
                else if student.has_small_car() && sml_tot < Sorter::MAX_SML{
                    this_day_results[student_idx] = String::from("SML");
                    sml_tot += 1;
                }
                else if reg_tot < Sorter::MAX_REG{
                    this_day_results[student_idx] = String::from("REG");
                    reg_tot += 1;
                }
                else{
                    this_day_results[student_idx] = String::from("BART");
                }
            }

            results.push(this_day_results);
            par_tot = 0;
            sml_tot = 0;
            reg_tot = 0;
            double_ability_students = VecDeque::new();
        }

        //transpose and return!
        let mut output: Vec<Vec<String>> = Vec::new();
        for s in 0..self.students[0].len()-1{
            let mut col: Vec<String> = Vec::new();
            for row in 0..5{ //there are 6 rows: name + 5 days
                col.push(results[row][s].clone());
            }
            output.push(col);//now col is a row, so transposed
        }
        println!("{:?}", results);
        println!("{:?}", output);
        output

    }
}

fn main() {
    println!("Hello, world!");
    let s = Sorter::new(vec![
        vec![Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
    ]);

    println!("{:?}", s.get_assignments());

}
