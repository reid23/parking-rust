mod Student;
mod Sorter;

fn main() {

    let s = Sorter::Sorter::new(vec![
        vec![Student::Student::new("bob", &vec![1,94303,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
        vec![Student::Student::new("bob", &vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1])],
    ]);

    println!("{:?}", s.get_assignments());

}
