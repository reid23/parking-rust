mod Student;
mod Sorter;

fn main() {

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
