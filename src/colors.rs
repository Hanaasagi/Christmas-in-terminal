extern crate ncurses;

use ncurses::{
    COLOR_PAIR,
    printw,
    mvprintw,
    attron,
    attroff,
    start_color,
    init_pair
};

use ncurses::{
    COLOR_BLACK,
    COLOR_RED,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_BLUE,
    COLOR_MAGENTA,
    COLOR_CYAN,
    COLOR_WHITE
};


#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(i16)]
pub enum Palette {
    Black    = 0,
    Red      = 1,
    Green    = 2,
    Yellow   = 3,
    Blue     = 4,
    Magenta  = 5,
    Cyan     = 6,
    White    = 7,
}


pub fn start_color_mode() {
    start_color();

    init_pair(Palette::Black   as i16, COLOR_BLACK,   COLOR_BLACK);
    init_pair(Palette::Red     as i16, COLOR_RED,     COLOR_BLACK);
    init_pair(Palette::Green   as i16, COLOR_GREEN,   COLOR_BLACK);
    init_pair(Palette::Yellow  as i16, COLOR_YELLOW,  COLOR_BLACK);
    init_pair(Palette::Blue    as i16, COLOR_BLUE,    COLOR_BLACK);
    init_pair(Palette::Magenta as i16, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(Palette::Cyan    as i16, COLOR_CYAN,    COLOR_BLACK);
    init_pair(Palette::White   as i16, COLOR_WHITE,   COLOR_BLACK);
}


pub fn mv_paint(y: i32, x: i32, s: &str, color: Palette) -> i32 {
    attron(COLOR_PAIR(color as i16));
    let rtn = mvprintw(y, x, s);
    attroff(COLOR_PAIR(color as i16));
    rtn
}

pub fn paint(s: &str, color: Palette) -> i32 {
    attron(COLOR_PAIR(color as i16));
    let rtn = printw(s);
    attroff(COLOR_PAIR(color as i16));
    rtn
}
