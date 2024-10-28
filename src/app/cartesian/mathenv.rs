#![allow(dead_code)]

use std::vec;

pub struct MathEnvironment {
    pub points: Vec<(String, f64, f64)>,
    pub variables: Vec<(String, f64)>,
}

impl MathEnvironment {
    pub fn new() -> Self {
        Self {
            points: vec![],
            variables: vec![],
        }
    }

    pub fn add_point(&mut self, name: &str, x: f64, y: f64) {
        self.points.push((name.to_string(), x, y));
    }

    pub fn add_variable(&mut self, name: &str, value: f64) {
        self.variables.push((name.to_string(), value));
    }

    pub fn get_value(&self, name: &str) -> Option<f64> {
        self.variables
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| *v)
    }

    pub fn get_point(&self, name: &str) -> Option<(f64, f64)> {
        self.points
            .iter()
            .find(|(n, _, _)| n == name)
            .map(|(_, x, y)| (*x, *y))
    }

    pub fn update_point(&mut self, name: &str, x: f64, y: f64) {
        if let Some(point) = self.points.iter_mut().find(|(n, _, _)| n == name) {
            point.1 = x;
            point.2 = y;
        }
    }
}
