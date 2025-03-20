pub mod utils;

use std::fmt::{self, Debug, Formatter};

use anyhow::Error;
// use polars;
// use polars::prelude::*;
use serde::{
    Deserialize,
    // Deserializer,
    Serialize,
    // Serializer, de::Visitor, ser::SerializeTuple
};
// use serde_json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PseudoTensor<T> {
    // shape
    // : shape is a vector of integers that represents the shape of the tensor.
    // : e.g. [2, 3, 4] means a tensor with 3 dimensions, and each dimension has 2, 3, and 4 elements.
    // :  first dimension has 2 elements, second dimension has 3 elements, and third dimension has 4 elements.
    #[serde(rename = "s")]
    pub shape: Vec<u8>,
    #[serde(rename = "r")]
    pub is_auto_shape: bool,
    #[serde(rename = "d")]
    pub data: Vec<PseudoTensorData<T>>,
}

impl<T> PseudoTensor<T> {
    pub fn new(shape: Vec<u8>) -> Self {
        Self {
            shape,
            data: vec![],
            is_auto_shape: false,
        }
    }

    pub fn new_with_data(shape: Vec<u8>, data: Vec<PseudoTensorData<T>>) -> Result<Self, Error> {
        Ok(Self {
            shape,
            data,
            is_auto_shape: false,
        })
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoTensorData<T> {
    #[serde(rename = "c")]
    pub coordinate: Vec<u64>,
    #[serde(rename = "v")]
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
