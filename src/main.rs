use raylib::prelude::*;

const WIDTH : i32 = 64;
const HEIGTH : i32 = 32;

fn main() 
{
    const MULTIPLIER : i32 = 10;
    const SCREEN_WIDTH : i32 = WIDTH * MULTIPLIER; 
    const SCREEN_HEIGTH : i32 = HEIGTH * MULTIPLIER ; 

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGTH)
        .title("chip 8 rs")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
    }
}