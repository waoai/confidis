use crate::command::Answer;
use crate::equalifier::Equalifier;

pub struct ExactEqualifier {}

impl ExactEqualifier {
    pub fn new() -> Self {
        ExactEqualifier {}
    }
}

impl Equalifier for ExactEqualifier {
    fn is_valid_answer(&self, _a: &Answer) -> bool {
        true
    }
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        return if a.content == b.content { 0.0 } else { 1.0 };
    }
}
