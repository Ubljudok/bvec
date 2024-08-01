use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct BVECBuffer {
    buffer: Vec<u16>, //Pixel data
    size_x: u16, //Number of pixels on the X Axis
    size_y: u16, //Number of pixels on the Y Axis
    changed: bool, //Whether the buffer had been changed since it was last checked
}

impl BVECBuffer {
    pub fn new(
        size_x: u16, 
        size_y: u16,
    ) -> BVECBuffer {
        todo!();
    }

    pub fn clear_screen(
        r: u8, //Red Value of Color
        g: u8, //Green Value of Color
        b: u8, //Blue Value of Color
        visible: bool, //Whether pixel can be seen or not (Unused)
    ) {
        todo!();
    }

    pub fn set_pixel(
        x: u16, //X Coordinate
        y: u16, //Y Coordinate
        r: u8, //Red Value of Color
        g: u8, //Green Value of Color
        b: u8, //Blue Value of Color
        visible: bool, //Whether pixel can be seen or not (Unused)
    ) {
        todo!();
    }

    pub fn draw_line(
        start_x: u16, //Beginning X Coordinate
        start_y: u16, //Beginning Y Coordinate
        end_x: u16, //End X Coordinate
        end_y: u16, //End Y Coordinate
        thickness: u8, //How many pixels thick to draw the line
        r: u8, //Red Value of Color
        g: u8, //Green Value of Color
        b: u8, //Blue Value of Color
        visible: bool, //Whether pixel can be seen or not (Unused)
    ) {
        todo!();
    }

    pub fn draw_poly(
        vertices: Vec<[u16; 2]>, //List of vertex coordinates
        indices: Vec<[u16; 2]>, //List of indices, which lines are drawn between
        thickness: u8, //How many pixels thick to draw the line
        r: u8, //Red Value of Color
        g: u8, //Green Value of Color
        b: u8, //Blue Value of Color
        visible: bool, //Whether pixel can be seen or not (Unused)
    ) {
        todo!();
    }

    pub fn draw_char(
        x: u16, //X Coordinate of top left corner
        y: u16, //Y Coordinate of top left corner
        font: u8, //Bitmap font of character
        r: u8, //Red Value of Color
        g: u8, //Green Value of Color
        b: u8, //Blue Value of Color
        visible: bool, //Whether pixel can be seen or not (Unused)
    ) {
        todo!();
    }

    pub fn draw_string(
        x: u16, //X Coordinate of top left corner
        y: u16, //Y Coordinate of top left corner
        font: u8, //Bitmap font of character
        r: u8, //Red Value of Color
        g: u8, //Green Value of Color
        b: u8, //Blue Value of Color
        visible: bool, //Whether pixel can be seen or not (Unused)
    ) {
        todo!();
    }
}
