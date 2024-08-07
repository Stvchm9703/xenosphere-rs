// use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoTensor<T> {
    pub shape: Vec<i32>,
    pub data: Vec<PseudoTensorData<T>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoTensorData<T> {
    pub position: Vec<i32>,
    pub value: T,
}

impl<T> PseudoTensorData<T> {
    pub fn new(position: Vec<i32>, value: T) -> Self {
        PseudoTensorData { position, value }
    }
}

pub fn merge_shape(a_shape: &Vec<i32>, b_shape: &Vec<i32>) -> Vec<i32> {
    if a_shape.len() == 0 {
        return b_shape.clone();
    }
    if b_shape.len() == 0 {
        return a_shape.clone();
    }
    let mut base_iter = a_shape.clone();
    let mut check_iter = b_shape.clone();
    if a_shape.len() > b_shape.len() {
        let mut cloned = b_shape.clone();
        cloned.resize(a_shape.len(), 0);
        check_iter = cloned;
    } else if b_shape.len() > a_shape.len() {
        let mut cloned = a_shape.clone();
        cloned.resize(b_shape.len(), 0);
        base_iter = cloned;
    }
    let zip_iter = base_iter.iter().zip(check_iter.iter());
    zip_iter
        .map(|(x, y)| if x < y { *y } else { *x })
        .collect::<Vec<i32>>()
    // base_iter.map(|x| *x).collect::<Vec<i32>>()
}
