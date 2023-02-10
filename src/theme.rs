use termimad::crossterm::style::{Color::*};//, Attribute::*};
use termimad::*;

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Rgb {r:77, g:255, b:77});
    
    //skin.set_bg(Rgb {r:0, g:0, b:0});

    skin //return
}