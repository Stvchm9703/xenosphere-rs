use std::fmt::{self, Debug, Formatter};

use anyhow::Error;
// use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PseudoTensor<T> {
    // shape
    // : shape is a vector of integers that represents the shape of the tensor.
    // : e.g. [2, 3, 4] means a tensor with 3 dimensions, and each dimension has 2, 3, and 4 elements.
    // :  first dimension has 2 elements, second dimension has 3 elements, and third dimension has 4 elements.
    pub shape: Vec<u8>,
    pub data: Vec<PseudoTensorData<T>>,
}

impl<T> PseudoTensor<T> {
    pub fn new(shape: Vec<u8>) -> Self {
        Self {
            shape,
            data: vec![],
        }
    }

    pub fn new_with_data(shape: Vec<u8>, data: Vec<PseudoTensorData<T>>) -> Result<Self, Error> {
        Ok(Self { shape, data })
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoTensorData<T> {
    pub coordinate: Vec<u64>,
    pub value: T,
}

impl<T> PseudoTensorData<T> {
    pub fn new(coordinate: Vec<u64>, value: T) -> Self {
        PseudoTensorData { coordinate, value }
    }
}

impl<T: Debug> Debug for PseudoTensorData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    where
        T: Debug,
    {
        return write!(f, "pt: {:?};{:?}", self.coordinate, self.value);
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
