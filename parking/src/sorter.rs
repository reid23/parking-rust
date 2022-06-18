use Student;

pub struct Sorter{
    mut students: Vec<Vec<Student>>,
    student_names: Vec<String>
}

impl Sorter{
    const MAX_REG: usize = 41;
    const MAX_SML: usize = 8;
    const MAX_PAR: usize = 4;
    
    pub fn new(all_students: Vec<Vec<Student>>) -> Sorter{
        let names: Vec<String> = Vec::new();
        for s in students{names.push(s.get_name())}
        names.sort();

        s = Sorter {
            students: students //should be [monday[student, student, ...], tuesday[student, student, student]]
            student_names: names
        }
        s.sort_days()
        
        s
    }

    fn sort_days(&self) -> None{
        for day in self.students{
            day.sort_by(|a| a.generate_score())
        }
    }

    fn index(&self, student: Student|String) -> usize{
        //! SLOW! but I don't have wifi to search up the builtin version ;-;
        let mut idx: usize = 0;
        for name in self.student_names{
            if (student == name || student.get_name() == name){
                return idx;
            }
            idx++;
        }
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
        let mut day_num: usize = 0;
        let mut par_tot: usize = 0;
        let mut sml_tot: usize = 0;
        let mut reg_tot: usize = 0;

        for day in self.students{
            let mut this_day_results: Vec<String> = vec![String::new(); day.len()];
            for student in day{
                let student_idx = self.index(student)
                if (student.can_parallel_park() && par_tot < Sorter::MAX_PAR){
                    results[student_idx] = String::from("PAR");
                    par_tot++;
                }
                else if (student.has_small_car() && sml_tot < Sorter::MAX_SML){
                    results[student_idx] = String::from("SML");
                    sml_tot++;
                }
                else if (reg_tot < Sorter::MAX_REG){
                    results[student_idx] = String::from("REG");
                    reg_tot++;
                }
                else{
                    results[student_idx] = String::from("BART");
                }
            }
            results.push(this_day_results);
            day_num++;
        }

        //then transpose and return

    }