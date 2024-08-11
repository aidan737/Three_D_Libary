extern crate piston_window;
pub use piston_window::*;




pub fn start_window(title:&str,width: f64, hight: f64) -> PistonWindow {
    let mut window: PistonWindow = 
    WindowSettings::new(title, [width, hight])
    .exit_on_esc(true).build().unwrap();

    return window;
}







