use std::fs;
use std::error::Error;
use serde_json::{Value, Map};
use rand_distr::{Normal, Distribution};

pub struct Student{
    pub name: String,
    pub row: Vec<usize>,
    pub dists: Map<String, Value>
}

impl Student{
    fn read_config(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }

    pub fn new(student_name: &str, row: &Vec<usize>) -> Student{
        Student {
            name: String::from(student_name),
            row: row.clone(),
            dists: Student::read_config("/Users/reiddye/parking-rust/distances.json").unwrap()
        }
    }

    pub fn generate_score(&self, day: usize) -> f32{
        let crit = self.row[2..7][day];
        let sports = self.row[7..12][day];
        let fp_free = self.row[14];
        let lp_free = self.row[15];
        let dist: f32 = self.dists.get(&self.row[1].to_string()).unwrap().as_f64().unwrap() as f32;
        let carpool_seniors = self.row[12];
        let carpool_youngns = self.row[13];
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