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