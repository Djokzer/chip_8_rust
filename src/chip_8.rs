//Imports
use std::fs::*;
use std::fs::File;
use std::io::Read;
use raylib::prelude::*;

//Font sprites
const CH8_FONT : [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

//Chip8 emulation
pub struct Chip8
{
    pub memory : [u8; 4096],    //4 kB of memory
    pub v : [u8; 16],           //16 8-bit general purpose registers (v0-vF)
    pub stack : Vec<u16>,       //Stack for 16-bit address
    pub sp : u16,              //Stack pointer
    pub delay_timer : u8,       //8-bit delay timer
    pub sound_timer : u8,       //8-bit sound timer
    pub input : [u8; 16],       //Hex keyboard (0-F)
    pub display : [u8; 64*32],  //64x32 Display (Black & White)
    pub opcode : u16,           //Opcode are two bytes long (Big ENDIAN)
    pub index_reg : u16,        //Index register
    pub pc : u16,               //Program counter
}

pub fn init_ch8()->Chip8
{
    Chip8
    {
        memory : [0; 4096],
        v : [0; 16],
        stack : vec![],
        sp : 0,
        delay_timer : 0,
        sound_timer : 0,
        input : [0; 16],
        display : [0; 64 * 32],
        opcode : 0,
        index_reg : 0,
        pc : 0x200, // Program counter starts at 0x200
    }
}

impl Chip8
{
    pub fn store_font(&mut self)
    {
        for i in 0..80 
        {
            self.memory[i] = CH8_FONT[i]; //Load the font in the memory
        }
    }

    pub fn load_rom(&mut self, rom_name : &str)
    {
        let mut f = File::open(&rom_name).expect("no file found");
        let metadata = metadata(&rom_name).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        for i in 0..buffer.len() 
        {
            self.memory[i + 512] = buffer[i];
        }
    }

    pub fn emulate(&mut self)
    {
        //println!("PC : {}", self.pc);
        //Fetch
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc+1) as usize] as u16); 
        self.pc = self.pc + 2;
        
        //Decode
        let op = (self.opcode & 0xF000) >> 12; 
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let n = self.opcode & 0x000F;
        let nn = self.opcode & 0x00FF;
        let nnn = self.opcode & 0x0FFF;
        
        //println!("opcode : 0x{:04x}", self.opcode);

        match op
        {
            0x0 => 
            {
                if self.opcode == 0x00E0
                {
                    //Clear Screen
                    self.clear_display();
                }
                else if self.opcode == 0x00EE
                {
                    //Return from subroutines
                    self.pc = self.stack.pop().unwrap();
                    self.sp -= 1;
                }
            }
            0x1 =>
            {   
                //Jump
                self.pc = nnn;
            }
            0x2 =>
            {
                //Subroutines
                self.stack.push(self.pc);
                self.sp += 1;
                self.pc = nnn;
            }
            0x3 =>
            {
                //Skip if v[x] == nn
                if self.v[x as usize] == nn as u8
                {
                    self.pc += 2;
                }
            }
            0x4 =>
            {
                //Skip if v[x] != nn
                if self.v[x as usize] != nn as u8
                {
                    self.pc += 2;
                }
            }
            0x5 =>
            {
                //Skip if v[x] == v[y]
                if self.v[x as usize] == self.v[y as usize]
                {
                    self.pc += 2;
                }
            }
            0x6 =>
            {
                //Set register vx
                self.v[x as usize] = nn as u8;
            }
            0x7 =>
            {
                //Add value to registe vx
                let mut temp : u16 = self.v[x as usize] as u16;
                temp += nn;
                self.v[x as usize] = (temp % 256) as u8;
            }
            0x8 =>
            {
                match self.opcode & 0x800F
                {
                    //Logical and arithmetic instructions
                    0x8000 =>
                    {
                        //Set
                        self.v[x as usize] = self.v[y as usize];
                    }
                    0x8001 =>
                    {
                        //Binary OR
                        self.v[x as usize] |= self.v[y as usize];
                    }
                    0x8002 =>
                    {
                        //Binary AND
                        self.v[x as usize] &= self.v[y as usize];
                    }
                    0x8003 =>
                    {
                        //Logical XOR
                        self.v[x as usize] ^= self.v[y as usize];
                    }
                    0x8004 =>
                    {
                        //ADD
                        let mut temp : u16 = self.v[x as usize] as u16;
                        temp += self.v[y as usize] as u16;

                        if temp > 255{self.v[15] = 1;} //Set carry flag
                        else{self.v[15] = 0;}

                        self.v[x as usize] = (temp % 256) as u8;
                    }
                    0x8005 =>
                    {
                        //SUBSTRACT vx = vx - vy
                        let mut tmp : i32 = self.v[x as usize] as i32 - self.v[y as usize] as i32;

                        if self.v[x as usize] > self.v[y as usize]
                        {
                            self.v[15] = 1; //Set carry flag
                        }
                        else
                        {
                            self.v[15] = 0;
                            tmp += 256; //To get a positive value 
                        }
                        
                        self.v[x as usize] = tmp as u8;
                    }
                    0x8007 =>
                    {
                        //SUBSTRACT vx = vy - vx
                        let mut tmp : i32 = self.v[y as usize] as i32 - self.v[x as usize] as i32;

                        if self.v[y as usize] > self.v[x as usize]
                        {
                            self.v[15] = 1; //Set carry flag
                        }
                        else
                        {
                            self.v[15] = 0;
                            tmp += 256; //To get a positive value 
                        }
                        
                        self.v[x as usize] = tmp as u8;
                    }
                    0x8006 =>
                    {
                        //SHIFT RIGHT vx = vy >> 1
                        self.v[x as usize] = self.v[y as usize];
                        
                        self.v[15] = self.v[x as usize] & 0x01; //Vf = shfted out bit
                        
                        self.v[x as usize] = self.v[x as usize] >> 1;
                    }
                    0x800E =>
                    {
                        //SHIFT LEFT vx = vy << 1
                        self.v[x as usize] = self.v[y as usize];
                        
                        self.v[15] = self.v[x as usize] & 0x80; //Vf = shfted out bit
                        
                        self.v[x as usize] = self.v[x as usize] << 1;
                    }
                    _=> println!("Unknown OPCODE 0x{:04x}", self.opcode),
                }
            }
            0x9 =>
            {
                //Skip if v[x] != v[y]
                if self.v[x as usize] != self.v[y as usize]
                {
                    self.pc += 2;
                }
            }
            0xA =>
            {
                //Set the index register to NNN
                self.index_reg = nnn;
            }
            0xB =>
            {
                self.pc = nnn + self.v[0] as u16;
            }
            0xC =>
            {
                let rng : i32 = get_random_value(0, 255);
                self.v[x as usize] = rng as u8 & nn as u8;
            }
            0xD =>
            {
                //Draw in the display
                let x_pos = self.v[x as usize] % 64;    //Get x coordinate
                let y_pos = self.v[y as usize] % 32;    //Get y coordinate
                self.v[15] = 0;                         //Clear Register v[f]

                for y_off in 0..n 
                {
                    let pixel = self.memory[self.index_reg as usize + y_off as usize];
                    for x_off in 0..8 as u16
                    {
                        if pixel & (0x80 >> x_off) != 0 //If x in pixel is On
                        {
                            if self.display[(x_pos as u16 + x_off + (y_pos as u16 + y_off) * 64) as usize] == 1
                            {
                                self.v[15] = 1;
                            }
                            self.display[(x_pos as u16 + x_off + (y_pos as u16 + y_off) * 64) as usize] ^= 1; //Xor the pixel
                        }  
                    }
                }
            }
            _ => println!("Unknown OPCODE 0x{:04x}", self.opcode),
        }
    }

    pub fn clear_display(&mut self)
    {
        for i in 0..self.display.len() 
        {
            self.display[i] = 0;
        }
    }
}