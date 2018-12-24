extern crate ncurses;
extern crate rand;
mod colors;


use ncurses::*;
use std::thread;
use std::time;
use rand::Rng;
use self::colors::*;


const MAX_LIGHT_GROUP: usize =  2;
const LIGHT_NUMBER: usize = 35;


fn get_screen_bounds() -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    return (max_y, max_x);
}


fn run() -> ! {
    initscr();
    start_color_mode();

    let (_, max_x) = get_screen_bounds();
    let mut x = max_x / 2;
    let mut y = 2;

    // Tree
    for i in (1..20).step_by(2) {
        mv_paint(y, x, "*", Palette::Green);
        for _ in 1..i {
            paint("*", Palette::Green);
        }
        y = y + 1;
        x = x - 1;
    }

    // Chunk
    x = max_x / 2;
    for _ in 1..3 {
        mv_paint(y, x-1, "mWm", Palette::Magenta);
        y = y + 1;
    }

    y += 1;
    mv_paint(y, (max_x / 2) - 6, "MERRY CHRISTMAS",  Palette::White);
    refresh();

    let mut cur_group = 0;

    // Mark the light has been lit
    let mut y_mark_arr = [[0_i32; LIGHT_NUMBER]; MAX_LIGHT_GROUP];
    let mut x_mark_arr  = [[0_i32; LIGHT_NUMBER]; MAX_LIGHT_GROUP];

    // exclude green color, black
    let colors = vec![
        Palette::Red,
        Palette::Yellow,
        Palette::Blue,
        Palette::Magenta,
        Palette::Cyan,
        Palette::White,
    ];

    let mut rng = rand::thread_rng();

    let mut warm = false;

    let half_sec: time::Duration  = time::Duration::from_millis(500);

    loop {
        for i in 0..LIGHT_NUMBER {

            if warm {
                // last group
                let mark_y = y_mark_arr[(cur_group - 1) % MAX_LIGHT_GROUP][i];
                let mark_x = x_mark_arr[(cur_group - 1) % MAX_LIGHT_GROUP][i];
                mv_paint(mark_y, mark_x, "*", Palette::Green);
            }

            let light_y = (rng.gen::<u8>() % 9 + 3) as i32;
            let start = max_x / 2 - light_y + 2;
            let light_x = (rng.gen::<u8>() as i32  % (light_y - 2) * 2 + 1 + start) as i32;

            // mark
            y_mark_arr[cur_group][i] = light_y;
            x_mark_arr[cur_group][i] = light_x;

            // choose color
            let color_idx = rng.gen_range(0, colors.len()) as usize;
            mv_paint(light_y, light_x, "o", colors[color_idx]);

            refresh();
            thread::sleep(half_sec);
        }
        cur_group = (cur_group + 1) % MAX_LIGHT_GROUP;
        warm = true;
    }
}

fn main() {
    run();
}
