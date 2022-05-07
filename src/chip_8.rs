//Imports

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
}