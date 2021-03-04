use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Pattern {
    weights: HashMap<String, u16>
}

impl Pattern {
    pub fn new() -> Pattern {
        let mut map = HashMap::new();
        Pattern {
            weights: map
        }
    }

    pub fn new_with(elements: &[(String, u16)]) -> Pattern {
        let mut pat = Pattern::new();
        pat.add_multiple(elements);
        pat
    }

    pub fn add(&mut self, string: String, weight: u16) {
        self.add_multiple(&[(string, weight)]);
    }

    pub fn add_multiple(&mut self, elements: &[(String, u16)]) {
        for (s, u) in elements {
            self.weights.insert(s.clone(), *u);
        }
    }

    pub fn to_percentages(&self) -> HashMap<String, u8> {
        let k: f32 = 100.0 / (self.weights.values().sum::<u16>() as f32);
        self.weights.iter().map(|t| (t.0.clone(), (*t.1 as f32 * k).round() as u8)).collect()
    }
}

impl TryFrom<String> for Pattern {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        let mut pattern = Pattern::new();

        // = "50%stone,10%dirt,15%glass"
        let elements = value.split(',');
        // = "50%stone" "10%dirt" "15%glass"
        for s in elements {
            // = "50" "stone"
            let components = s.split('%').collect::<Vec<&str>>();
            if components.len() != 2 {
                continue;
            }
            //todo add checks for if block is 'placeable'
            let weight_result = components.get(0).unwrap().parse::<u16>();
            if weight_result.is_err() {
                continue;
            }
            let string = components.get(1).unwrap();
            let weight = weight_result.unwrap();

            pattern.add(String::from(string.parse::<String>().unwrap()), weight);
        }

        return if pattern.weights.is_empty() {
            Err("No valid patterns found. String cannot be converted to pattern.")
        } else {
            Ok(pattern)
        }
    }
}