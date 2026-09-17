use std::error;
use std::fmt;

pub trait KrafError: error::Error + fmt::Display {}
