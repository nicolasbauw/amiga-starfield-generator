//use std::io;
use rand::Rng;

fn main() {
    /*let mut line = String::new();
    println!("Starting line (decimal)?");
    io::stdin().read_line(&mut line)
        .expect("Read line failed.");

    let starting_line = line
        .trim()
        .parse::<u8>()
        .unwrap();

    line = String::new();
    println!("Ending line (decimal)?");
    io::stdin().read_line(&mut line)
        .expect("Read line failed.");

    let ending_line = line
        .trim()
        .parse::<u8>()
        .unwrap();

    line = String::new();
    println!("Stars line interval (decimal)?");
    io::stdin().read_line(&mut line)
        .expect("Read line failed.");

    let line_interval = line
        .trim()
        .parse::<u8>()
        .unwrap();*/
        

    let mut starting_line: u16 = 0x95;
    let ending_line = 0xFE;
    let line_interval = 4;
    let mut rng = rand::thread_rng();

    while starting_line<ending_line{
    let vertical_add = rng.gen_range(0..line_interval);
    let horizontal_pos = rng.gen_range(0x40..0xDF);
    let color = rng.gen_range(0..3);

    generate_line(starting_line, horizontal_pos, color);
    starting_line+=vertical_add+2;
    }
    println!("0x0000,0x0000");
}

fn generate_line(starting_line: u16, horizontal_pos: u16, color: u16) {
    print!("{:#04X}", starting_line);
    print!("{:02X}", horizontal_pos);
    println!(",{:#02X}00,", starting_line+1);
    match color {
        0 => { println!("0x8000,0x0000,"); },
        1 => { println!("0x0000,0x8000,"); },
        2 => { println!("0x8000,0x8000,"); },
        _ => {}
    }
}

// Sprite positions
// horizontal left = $40 - right  = $DF
// vertical   top  = $2C - bottom = $FE
