use crate::utils::bit_registers::{bit_at, set_bit_at};

use std::os::raw::c_int;

pub struct StatusTool;
impl StatusTool {
    pub fn new() -> StatusTool {
        StatusTool {}
    }
}
