use Student;

pub struct Sorter{
    mut students: Vec<Vec<Student>>
}

impl Sorter{
    const MAX_REG: usize = 41;
    const MAX_SML: usize = 8;
    const MAX_PAR: usize = 4;
    
    pub fn new(all_students: Vec<Vec<Student>>) -> Sorter{
        s = Sorter {
            students: students //should be [monday[student, student, ...], tuesday[student, student, student]]
        }
        s.sort_days()
        
        s
    }

    fn sort_days(&self) -> None{
        for day in self.students{
            day.sort_by(|a| a.generate_score())
        }
    }

    fn index(&self, day: usize, student: Student) -> usize{
        for student in self.students[day]{
            match student{
                
            }
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

        for day in self.students{
            let mut this_day_results: Vec<String> = vec![String::new(); day.len()];
            for student in day{
                if student.can_parallel_park(){
                    results.
                }
            }
            results.push(this_day_results);
        }

        //then transpose and return

    }