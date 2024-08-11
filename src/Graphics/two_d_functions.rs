extern crate piston_window;

pub use piston_window::*;

pub struct Point2d {
   pub point_x: f64,
   pub point_y: f64,
}

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub transperency: f32,
 }

pub fn draw_rectangle(c: &Context, g: &mut G2d,position: &Point2d, color: &Color, width: f64, hight: f64) {
    rectangle([color.red,color.green, color.blue, color.transperency], // red
              [position.point_x, position.point_y, width, hight],
              c.transform, g);
}
pub fn clear_screen(c: &Context, g: &mut G2d) {
    clear([1.0; 4], g);
}


pub fn draw_pixel(c: &Context, g: &mut G2d,position: &Point2d, color: &Color) {
    rectangle([color.red,color.green, color.blue, color.transperency], // red
              [position.point_x, position.point_y, 1.0, 1.0],
              c.transform, g);
}
fn get_first_digit(mut num: u8, mut dived_value: u8) -> u8 {
    while num >= dived_value {
      num /= dived_value;
    }
    num
  }
  pub fn draw_image(c: &Context, g: &mut G2d, colors: &[Color],position: &Point2d, grid_x: u32, grid_y: u32, scale: f64) -> Result<(), String> {
    if colors.len() != (grid_x * grid_y) as usize {
        return Err(format!("Color array size ({}) doesn't match grid dimensions ({}x{})", colors.len(), grid_x, grid_y));
    }

    for y in 0..grid_y {
        for x in 0..grid_x {
            let index = (y * grid_x + x) as usize;
            if index >= colors.len() {
                // Handle potential index out-of-bounds (optional)
                return Err(format!("Index out of bounds: {}", index));
            }
            let color = &colors[index];
            let pos = Point2d {
                point_x: ((x as f64)*scale)+position.point_x, // Convert to f64 for drawing
                point_y: ((y as f64)*scale)+position.point_y,
            };
            draw_rectangle(&c, g, &pos, color,scale,scale); // Propagate potential errors
        }
    }

    Ok(())
}

pub fn draw_tryangle(c: &Context, g: &mut G2d,position1: &Point2d,position2: &Point2d,position3: &Point2d, color: [f32; 4]) {
    let vertices: [[f64; 2]; 3] = [[position1.point_x, position1.point_y], [position2.point_x, position2.point_y], [position3.point_x, position3.point_y]];

    polygon(color, &vertices, c.transform, g);
}


pub fn is_point_in_rectangle(point: &Point2d, rect_point: &Point2d,size: &Point2d) -> bool {
    let x = point.point_x;
    let y = point.point_y;
    let rect_x = rect_point.point_x;
    let rect_y = rect_point.point_y;
    let rect_width = size.point_x;
    let rect_height = size.point_y;

    if x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height {
        return true;
    }

    false
}

pub fn draw_text(c: &Context, g: &mut G2d, text: &str, position: &Point2d, color: &Color, scale: f64) {
    let mut cursor_x = position.point_x;
    let cursor_y = position.point_y;

    for char in text.chars() {
        draw_char(c, g, char, &Point2d { point_x: cursor_x, point_y: cursor_y }, color, scale);
        cursor_x += 6.0 * scale; // Move cursor for next character
    }
}

fn draw_char(c: &Context, g: &mut G2d, char: char, position: &Point2d, color: &Color, scale: f64) {
    let bitmap = get_char_bitmap(char);
    for (y, row) in bitmap.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            if pixel {
                let pixel_pos = Point2d {
                    point_x: position.point_x + (x as f64 * scale),
                    point_y: position.point_y + (y as f64 * scale),
                };
                draw_rectangle(&c, g, &pixel_pos, color,scale,scale);
            }
        }
    }
}

fn get_char_bitmap(char: char) -> [[bool; 5]; 7] {
    match char {
        'A' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'B' => [
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
        ], 
        'C' => [
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [true, true, true, true, false],
        ], 
        'D' => [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
        ], 
        'E' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ], 
        'F' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ], 
        'G' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, true, true, true],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ], 
        'H' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ], 
        'I' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, true, true, true],
        ], 

        'J' => [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ], 
        'K' => [
            [true, false, false, false, true],
            [true, false, false, true, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false],
        ], 
        'L' => [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
        ], 
        'M' => [
            [true, false, false, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ], 
        'N' => [
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, false, true, true],
            [true, false, false, false, true],
        ], 
        'O' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ], 
        'P' => [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ], 
        'Q' => [
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, true],
        ], 
        'R' => [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],  
        'S' => [
            [false, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ],  
        'T' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],  
        'U' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ], 
        'V' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ], 
        'W' => [
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, true, false],
        ], 
        'X' => [
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
        ], 
        'Y' => [
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ], 
        'Z' => [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [true, true, true, true, true],
        ],
        'a' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'b' => [
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
        ], 
        'c' => [
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [true, true, true, true, false],
        ], 
        'd' => [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
        ], 
        'e' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ], 
        'f' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ], 
        'g' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, true, true, true],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ], 
        'h' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ], 
        'i' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, true, true, true],
        ], 

        'j' => [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ], 
        'k' => [
            [true, false, false, false, true],
            [true, false, false, true, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false],
        ], 
        'l' => [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
        ], 
        'm' => [
            [true, false, false, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ], 
        'n' => [
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, false, true, true],
            [true, false, false, false, true],
        ], 
        'o' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ], 
        'p' => [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ], 
        'q' => [
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, true],
        ], 
        'r' => [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],  
        's' => [
            [false, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, true, true, true, false],
        ],  
        't' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],  
        'u' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ], 
        'v' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ], 
        'w' => [
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [true, false, true, false, true],
            [false, true, false, true, false],
        ], 
        'x' => [
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
        ], 
        'y' => [
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ], 
        'z' => [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [true, true, true, true, true],
        ],
        '1' => [
            [false, false, true, false, false],
            [false, true, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, true, true, false],
        ],
        '2' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [false, false, false, false, true],
            [false, false, true, true, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, true, true, true],
        ],
        '3' => [
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, true, true, true, true],
        ],
        '4' => [
            [false, false, false, true, true],
            [false, false, false, false, true],
            [false, false, true, false, true],
            [false, true, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
        ],
        '5' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, true, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [true, true, true, false, false],
        ],
        '6' => [
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, true, false],
        ],
        '7' => [
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, true, false, false],
        ],
        '8' => [
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
        ],
        '9' => [
            [false, true, true, true, true],
            [false, true, false, false, true],
            [false, true, false, false, true],
            [false, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, true, true, true, false],
        ],
        '0' => [
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
        ],
        '.' => [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, true, false, false, false],
            [true, true, false, false, false],
        ],
        // Add more characters here...
        _ => [[false; 5]; 7], // Default empty character
    }
}