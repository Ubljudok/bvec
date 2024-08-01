use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct BVECBuffer {
    buffer: Vec<u16>, //Pixel data
    size_x: u16, //Number of pixels on the X Axis
    size_y: u16, //Number of pixels on the Y Axis
}
