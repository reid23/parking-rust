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
        let mut double_ability_students: Vec<Student> = Vec::new()

        for day in self.students{
            let mut this_day_results: Vec<String> = vec![String::new(); day.len()];
            for student in day{
                let student_idx = self.index(student)
                
                //catching the corner case! from the python code:
                //>  # problem: if someone can park in par and small, they get put in par, but if there's someone 
                //>  # else later who can park in par but not sml, and par and reg is full, but sml is not, they'd 
                //>  # get put in barts even when they can park on campus
                if student.can_parallel_park() && student.has_small_car(){
                    double_ability_students.push(student);
                }
                if {student.can_parallel_park()&& //if the student can parallel park
                    reg_tot >= Sorter::MAX_REG && //but reg
                    par_tot >= Sorter::MAX_PAR && //and par are full
                    sml_tot <  Sorter::MAX_SML && // and sml isn't
                    double_ability_students.len() != 0}{ //and there's someone who could be moved from par to sml to allow this person to park
                    results[self.index(double_ability_students.pop(0))] = String::from("SML"); //TODO: check that pop works with idx like this
                    results[student_idx] = "PAR"
                    sml_tot++;
                }
                //ok that should be taken care of now, we can sort normally

                else if (student.can_parallel_park() && par_tot < Sorter::MAX_PAR){
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
            par_tot = 0;
            sml_tot = 0;
            reg_tot = 0;
            double_ability_students = Vec::new();
        }

        //transpose and return!
        let mut output: Vec<Vec<String>> = Vec::new();
        for s in 0..self.students[0].len(){
            let col: Vec<String> = Vec::new();
            for row in 0..6{ //there are 6 rows: name + 5 days
                col.push(self.students[row][s]);
            }
            output.push(col);//now col is a row, so transposed
        }

        output

    }