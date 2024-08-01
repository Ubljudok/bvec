use std::error::Error;
use std::fmt::Debug;

use crate::buffer::BVECBuffer;

#[derive(Debug)]
pub struct BVEC {
    buffer: BVECBuffer,
}
