use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use std::fs;

mod chip_8;

const WIDTH : i32 = 64;
const HEIGTH : i32 = 32;

fn main() 
{
    //WINDOW
    const MULTIPLIER : i32 = 10;
    const SCREEN_WIDTH : i32 = WIDTH * MULTIPLIER; 
    const SCREEN_HEIGTH : i32 = HEIGTH * MULTIPLIER ; 

    let (mut rl, thread) = raylib::init()
    .size(SCREEN_WIDTH, SCREEN_HEIGTH)
    .title("chip_8 rust")
    .build();

    //MAIN MENU
    let mut rom_choosed : bool = false;
    let paths = fs::read_dir("roms/")
    .unwrap()
    .filter_map(|e| e.ok())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect::<Vec<_>>();
    let mut rom_counter = 0;

    //CHIP-8
    let mut ch_8 = chip_8::init_ch8(); //Init the chip8
    let mut clk : f32 = 0.0;
    let mut timer_clk : f32 = 0.0;

    //MAIN LOOP
    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::get_color(0x2d3436)); //

        if !rom_choosed
        {
            //MAIN MENU
            main_menu(&mut d, SCREEN_WIDTH, SCREEN_HEIGTH, &mut ch_8, &paths, &mut rom_counter, &mut rom_choosed);
        }
        else
        {
            //Get Key press for emulation
            get_keypress(&mut d, &mut ch_8);

            //GET TIME
            clk += d.get_frame_time();
            timer_clk += d.get_frame_time();

            //500 HZ CLOCK
            if clk >= 0.002
            {
                //EMULATION CYCLE
                ch_8.emulate();
                clk = 0.0;
            }

            //60 HZ TIMER UPDATE
            if timer_clk >= 0.016
            {
                //UPDATE THE TIMERS
                ch_8.update_timer();
                timer_clk = 0.0;
            }

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
        }
    } 
}

fn main_menu(h : &mut RaylibDrawHandle, screen_width : i32, screen_height : i32, ch_8 : &mut chip_8::Chip8, path : &Vec<String>, rom_counter : &mut usize, rom_choosed : &mut bool)
{
    //DRAW MAIN MENU
    let text_width : i32 = measure_text(&path[*rom_counter], 40);
    h.draw_text(&path[*rom_counter], screen_width/2 - (text_width/2), screen_height / 2 - 20, 40,Color::WHITE);
    draw_arrow(h, screen_width, screen_height, Color::WHITE);

    //CHOOSE ROM
    if h.is_key_pressed(KEY_ENTER)
    {
                ch_8.store_font(); //Store the custom font in the memory
                ch_8.load_rom(&path[*rom_counter]);
                *rom_choosed = true;
    }
    else if h.is_key_pressed(KEY_RIGHT)
    {
        if path.len() - 1 > *rom_counter
        {
            *rom_counter+=1;
        }
    }
    else if h.is_key_pressed(KEY_LEFT)
    {
        if 0 < *rom_counter
        {
            *rom_counter-=1;
        }
    }
}

fn draw_bigger_pixel(h : &mut RaylibDrawHandle, x : i32, y : i32, mult : i32, color : Color)
{
    h.draw_rectangle(x * mult, y * mult, mult, mult, color);
}

fn draw_arrow(h : &mut RaylibDrawHandle, screen_width : i32, screen_height : i32, color : Color)
{
    let x_1 = screen_width as f32 - 20.0;
    let y_1 = screen_height as f32 / 2.0;

    let v1 : Vector2 = Vector2::new(x_1, y_1);
    let v2 : Vector2 = Vector2::new(x_1-20.0, y_1-20.0);
    let v3 : Vector2 = Vector2::new(x_1-20.0, y_1+20.0);

    h.draw_triangle(v1, v2, v3, color);

    let x_2 = 20.0;
    let y_2 = screen_height as f32 / 2.0;

    let v4 : Vector2 = Vector2::new(x_2, y_2);
    let v5 : Vector2 = Vector2::new(x_2+20.0, y_2+20.0);
    let v6 : Vector2 = Vector2::new(x_2+20.0, y_2-20.0);

    h.draw_triangle(v4, v5, v6, color);
}

fn get_keypress(h : &mut RaylibDrawHandle, ch_8 : &mut chip_8::Chip8)
{
    //FIRST ROW OF KEYS
    if h.is_key_down(KEY_ONE){ch_8.input[0x1] = 1;}
    else{ch_8.input[0x1] = 0;}

    if h.is_key_down(KEY_TWO){ch_8.input[0x2] = 1;}
    else{ch_8.input[0x2] = 0;}

    if h.is_key_down(KEY_THREE){ch_8.input[0x3] = 1;}
    else{ch_8.input[0x3] = 0;}

    if h.is_key_down(KEY_FOUR){ch_8.input[0xc] = 1;}
    else{ch_8.input[0xc] = 0;}

    //SECOND ROW OF KEYS
    if h.is_key_down(KEY_Q){ch_8.input[0x4] = 1;}
    else{ch_8.input[0x4] = 0;}

    if h.is_key_down(KEY_W){ch_8.input[0x5] = 1;}
    else{ch_8.input[0x5] = 0;}

    if h.is_key_down(KEY_E){ch_8.input[0x6] = 1;}
    else{ch_8.input[0x6] = 0;}

    if h.is_key_down(KEY_R){ch_8.input[0xd] = 1;}
    else{ch_8.input[0xd] = 0;}

    //THIRD ROW OF KEYS
    if h.is_key_down(KEY_A){ch_8.input[0x7] = 1;}
    else{ch_8.input[0x7] = 0;}

    if h.is_key_down(KEY_S){ch_8.input[0x8] = 1;}
    else{ch_8.input[0x8] = 0;}

    if h.is_key_down(KEY_D){ch_8.input[0x9] = 1;}
    else{ch_8.input[0x9] = 0;}

    if h.is_key_down(KEY_F){ch_8.input[0xe] = 1;}
    else{ch_8.input[0xe] = 0;}

    //FOURTH ROW OF KEYS
    if h.is_key_down(KEY_Y){ch_8.input[0xa] = 1;}
    else{ch_8.input[0xa] = 0;}

    if h.is_key_down(KEY_X){ch_8.input[0x0] = 1;}
    else{ch_8.input[0x0] = 0;}

    if h.is_key_down(KEY_C){ch_8.input[0xb] = 1;}
    else{ch_8.input[0xb] = 0;}

    if h.is_key_down(KEY_V){ch_8.input[0xf] = 1;}
    else{ch_8.input[0xf] = 0;}

}