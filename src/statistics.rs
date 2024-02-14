use std::collections::HashMap;
use std::fmt::Debug;

pub trait Accumulator: Debug {
    fn init(&self, i: usize) -> Statistic;

    fn accumulator(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct Mean {
    values: Vec<f64>,
    column_idx: usize,
}

impl Mean {
    pub fn new(_tmp: usize) {
        // TODO: validate against num_items
    }
}

impl Accumulator for Mean {
    fn init(&self, i: usize) -> Statistic {
        Statistic {
            value: *self.values.get(i).unwrap(),
        }
    }

    fn accumulator(&self) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Statistic {
    pub(crate) value: f64,
}

impl Statistic {
    pub fn new() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Statistics(pub(crate) HashMap<String, Statistic>);

impl Statistics {
    pub fn new(stats: HashMap<String, Statistic>) -> Self {
        Self(stats)
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
