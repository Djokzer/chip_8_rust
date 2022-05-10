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

    ch_8.load_rom("roms/IBM_Logo.ch8");

    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);

        //EMULATE
        ch_8.emulate();

        //DRAW
        for i in 0..ch_8.display.len() as i32
        {
            if ch_8.display[i as usize] != 0
            {
                draw_bigger_pixel(&mut d, i%64, i/64, MULTIPLIER, Color::WHITE);
            }
            else
            {
                draw_bigger_pixel(&mut d, i%64, i/64, MULTIPLIER, Color::BLACK);
            }
        }

        //Get Key press

    } 
}

fn draw_bigger_pixel(h : &mut RaylibDrawHandle, x : i32, y : i32, mult : i32, color : Color)
{
    h.draw_rectangle(x * mult, y * mult, mult, mult, color);
}