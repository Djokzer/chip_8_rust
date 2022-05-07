use raylib::prelude::*;

mod chip_8;

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

    let mut ch_8 = chip_8::init_ch8(); //Init the chip8
    ch_8.store_font(); //Store the custom font in the memory

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        //EMULATE

        //DRAW
        d.clear_background(Color::WHITE);

        //Get Key press
    }
}