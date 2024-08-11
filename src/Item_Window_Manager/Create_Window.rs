#[path = "../Graphics/two_d_functions.rs"] mod two_d_functions;
use two_d_functions::*;

#[derive(Clone)]
pub struct Window_feild
{
    pub input_type_string1_int2: i8,
    pub title: String,
    pub value: String,
}

pub struct Window_Struct
{
    pub point_x: f64,
    pub point_y: f64,
}



pub fn draw_window(c: &Context, g: &mut G2d,feilds: &mut Vec<Window_feild>, window_info: &mut Window_Struct,mouse_position:[f64; 2], mouse:bool, width:f64, hight:f64, title: String,keypressed:Key) -> Vec<Window_feild>
{
   
    //draw top of the window
    draw_rectangle(
        &c,
        g,
        &Point2d {
            point_x: window_info.point_x ,
            point_y: window_info.point_y,
           },                
        &Color {
            red: 0.5,
            green: 162.0,
            blue: 237.0,
            transperency: 1.0,
        },
        width,
        hight
        
    );



    let mut responses = feilds.clone();
    for (index, field) in feilds.iter().enumerate() {

      if field.input_type_string1_int2 == 1 
      {
        //this is the backing for the text box element


        draw_rectangle(
            &c,
            g,
            &Point2d {
                point_x: window_info.point_x ,
                point_y: window_info.point_y+((index as f64+1.0)*hight ),
               },                
            &Color {
                red: 0.0,
                green: 162.0,
                blue: 237.0,
                transperency: 1.0,
            },
            width,
            hight
            
        );
        draw_text(&c, g, &field.title, &Point2d {
            point_x: window_info.point_x ,
            point_y: window_info.point_y+((index as f64+1.0)*hight ),
           },  &Color {
            red: 100.0,
            green: 0.0,
            blue: 0.0,
            transperency: 1.0,
        }, 5.0);


      }
      if field.input_type_string1_int2 == 2
      {
        responses[index].value =   draw_textbox_numbers(&c, g, &Point2d {
            point_x: window_info.point_x ,
            point_y: window_info.point_y+((index as f64+1.0)*hight ),
           },   &Point2d {
            point_x: width,
            point_y: hight,
           }, keypressed, &responses[index].value,&index,mouse_position, mouse
        );
      

      }
      if field.input_type_string1_int2 == 3
      {
        //this is the backing for the button element


        draw_rectangle(
            &c,
            g,
            &Point2d {
                point_x: window_info.point_x ,
                point_y: window_info.point_y+((index as f64+1.0)*hight ),
               },                
            &Color {
                red: 0.0,
                green: 162.0,
                blue: 237.0,
                transperency: 1.0,
            },
            width,
            hight
            
        );
        draw_text(&c, g, &field.title, &Point2d {
            point_x: window_info.point_x ,
            point_y: window_info.point_y+((index as f64+1.0)*hight ),
           },  &Color {
            red: 100.0,
            green: 0.0,
            blue: 0.0,
            transperency: 1.0,
        }, 1.0);
        
        if(is_point_in_rectangle(
            &Point2d {
             point_x: mouse_position[0] ,
             point_y: mouse_position[1],
            },
            &Point2d {
                point_x: window_info.point_x ,
                point_y: window_info.point_y+((index as f64+1.0)*hight ),
               },
            &Point2d {
             point_x: width,
             point_y: hight,
            } ) && mouse){
                responses[index].value = "true".to_string();
            }
      }


    }

    //draw the title last to be infrount of the other elements
    draw_text(&c, g, &title, &Point2d {
        point_x: window_info.point_x ,
        point_y: window_info.point_y,
       },  &Color {
        red: 100.0,
        green: 0.0,
        blue: 0.0,
        transperency: 1.0,
    }, 2.0);
 //handel mouse movement bottom to allow for button presses
 if(mouse){
    if(is_point_in_rectangle(
        &Point2d {
         point_x: mouse_position[0] ,
         point_y: mouse_position[1],
        },
        &Point2d {
            point_x: window_info.point_x ,
            point_y: window_info.point_y,
           },
        &Point2d {
         point_x: width,
         point_y: hight,
        } )){
            *window_info =Window_Struct {
                point_x: mouse_position[0]-(width/2.0),
                point_y: mouse_position[1]-(hight/2.0),
               };
        }

    }
    responses
}

use std::sync::Mutex;
static is_key_down: Mutex<bool> = Mutex::new(false);

static textbox_held: Mutex<usize> = Mutex::new(0);


fn draw_textbox(c: &Context, g: &mut G2d,position: &Point2d, size: &Point2d, keypressed: Key,current_str: &String) -> String
{

    let mut updated_str = current_str.to_string();
    let mut key_down = is_key_down.lock().unwrap();
    
    match keypressed {
        Key::Backspace => {
            if(*key_down == false){
            updated_str.pop();
            *key_down = true;
            }
        },
        Key::Unknown => {
            *key_down = false;
        },
        Key::Return => {
            // Do nothing, or handle Enter key as needed
        },
        _ => {
            if(*key_down == false){
            if let Some(ch) = key_to_char(keypressed) {
                *key_down = true;
                updated_str.push(ch);
            }
        }
        },
    }
   

    draw_text(&c, g, &updated_str, position,  &Color {
        red: 100.0,
        green: 0.0,
        blue: 0.0,
        transperency: 1.0,
    }, 2.0);
   return(updated_str);
}


fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),
        Key::D0 => Some('0'),
        Key::D1 => Some('1'),
        Key::D2 => Some('2'),
        Key::D3 => Some('3'),
        Key::D4 => Some('4'),
        Key::D5 => Some('5'),
        Key::D6 => Some('6'),
        Key::D7 => Some('7'),
        Key::D8 => Some('8'),
        Key::D9 => Some('9'),
        Key::Space => Some(' '),
        // Add any other keys you want to support
        _ => None,
    }
}
fn draw_textbox_numbers(c: &Context, g: &mut G2d,position: &Point2d, size: &Point2d, keypressed: Key,current_str: &String, index:&usize,mouse_position:[f64; 2], mouse:bool) -> String
{
  
    let mut updated_str = current_str.to_string();
    let mut key_down = is_key_down.lock().unwrap();
    let mut textbox_held_index = textbox_held.lock().unwrap();


    if(mouse){
        if(updated_str == "0".to_string())
        {
            updated_str= "".to_string();
        }
        if(is_point_in_rectangle(
            &Point2d {
             point_x: mouse_position[0],
             point_y: mouse_position[1],
            },
            position,
            size)){
               *textbox_held_index = (index +1)
            }
            if(!is_point_in_rectangle(
                &Point2d {
                 point_x: mouse_position[0] ,
                 point_y: mouse_position[1],
                },
                position,
                size)){
                    if(  *textbox_held_index == (index +1)){
                   *textbox_held_index = 0;
                    }
                }
        }

    if((index +1)== *textbox_held_index){
    match keypressed {
        Key::Backspace => {
            if(*key_down == false){
            updated_str.pop();
            *key_down = true;
            }
        },
        Key::Unknown => {
            *key_down = false;
        },
        Key::Return => {
            // Do nothing, or handle Enter key as needed
        },
        _ => {
            if(*key_down == false){
              
            if let Some(ch) = num_to_char(keypressed) {
                *key_down = true;
                //checking for only 1 fullstop in number
                if keypressed == Key::Period {
                    if(count_char_occurrences(&updated_str, '.') == 0){
                    updated_str.push('.');
                    }
                }
                else
                {
                    updated_str.push(ch);

                }
            }
            
        }
        },
    }
 
  
    
}else{
if(updated_str == "".to_string())
{
    updated_str= "0".to_string();
}
}
draw_text(&c, g, &updated_str, position,  &Color {
    red: 100.0,
    green: 0.0,
    blue: 0.0,
    transperency: 1.0,
}, 2.0);
   return(updated_str);
}
fn count_char_occurrences(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}
fn num_to_char(key: Key) -> Option<char> {
    match key {   
        Key::D0 => Some('0'),
        Key::D1 => Some('1'),
        Key::D2 => Some('2'),
        Key::D3 => Some('3'),
        Key::D4 => Some('4'),
        Key::D5 => Some('5'),
        Key::D6 => Some('6'),
        Key::D7 => Some('7'),
        Key::D8 => Some('8'),
        Key::D9 => Some('9'),
        Key::Period => Some('.'),
        
        // Add any other keys you want to support
        _ => None,
    }
}
