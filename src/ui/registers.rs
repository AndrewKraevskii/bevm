use crate::model::{Computer, Register};
use std::os::raw::c_int;

pub struct RegistersTool;

impl RegistersTool {
    pub fn new() -> RegistersTool {
        RegistersTool {}
    }
}
