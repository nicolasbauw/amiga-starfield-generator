use std::io;
use rand::Rng;

fn main() {
    let mut line = String::new();
    println!("Starting line (decimal - min = 44)?");
    io::stdin().read_line(&mut line)
        .expect("Read line failed.");

    let mut starting_line = line
        .trim()
        .parse::<u8>()
        .expect("Cannot convert.");

    line = String::new();
    println!("Ending line (decimal - max = 298)?");
    io::stdin().read_line(&mut line)
        .expect("Read line failed.");

    let ending_line = line
        .trim()
        .parse::<u8>()
        .expect("Cannot convert.");

    line = String::new();
    println!("Stars line interval (decimal)?");
    io::stdin().read_line(&mut line)
        .expect("Read line failed.");

    let line_interval = line
        .trim()
        .parse::<u8>()
        .expect("Cannot convert.");
        
    let mut speeds: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();

    while starting_line<ending_line{
    let vertical_add = rng.gen_range(0..line_interval);
    let horizontal_pos = rng.gen_range(0x00..0xDF);
    let color = rng.gen_range(0..3);

    generate_line(starting_line, horizontal_pos, color, &mut speeds);
    starting_line = starting_line.wrapping_add(vertical_add+2);
    }
    println!("0x0000,0x0000");
    println!("\nSpeeds table:\n{:?}", speeds);
    println!("{} stars generated.", speeds.len());
}

fn generate_line(starting_line: u8, horizontal_pos: u8, color: u8, speeds: &mut Vec<u8>) {
    print!("{:#04X}", starting_line);
    print!("{:02X}", horizontal_pos);
    println!(",{:#02X}00,", starting_line+1);
    match color {
        0 => { println!("0x8000,0x0000,"); speeds.push(1) },
        1 => { println!("0x0000,0x8000,"); speeds.push(2) },
        2 => { println!("0x8000,0x8000,"); speeds.push(3) },
        _ => {}
    }
}

// Sprite positions
// horizontal           $40 -> $DF
// vertical             $2C -> $FE
// vertical (after $FF) $00 -> $2C
//
// Sprite data word
// vstart, hstart/2, vstop
// color data
//
// Sample control words after vpos $FF
// 0x0341,0x0406,
//             |---> %00000110