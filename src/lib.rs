use colored::*;

use rand::Rng;
use std::io;

pub fn generate_symbol() {
    const NUM_SYM: usize = 14;
    let symbols: [char; NUM_SYM] = [
        // smile with heart eyes
        char::from_u32(128525).unwrap(),
        // kiss smile
        char::from_u32(128536).unwrap(),
        // sun
        char::from_u32(127774).unwrap(),
        // sunflower
        char::from_u32(127803).unwrap(),
        // rose
        char::from_u32(127801).unwrap(),
        // little chicken
        char::from_u32(128037).unwrap(),
        // penguin
        char::from_u32(128039).unwrap(),
        // turtle
        char::from_u32(128034).unwrap(),
        // bear
        char::from_u32(128059).unwrap(),
        // beating heart
        char::from_u32(128147).unwrap(),
        // two hearts
        char::from_u32(128149).unwrap(),
        // sparkling heart
        char::from_u32(128150).unwrap(),
        // growing heart
        char::from_u32(128151).unwrap(),
        // revolving hearts
        char::from_u32(128158).unwrap()
    ];

    let r = rand::thread_rng().gen_range(1..NUM_SYM);
    let sym =  &symbols[r];
    println!("          {}\n", sym);
}

pub fn print_koala() {
    let koala = char::from_u32(128040).unwrap();
    println!("          {koala}");
    let mogree = "Mogree".to_string();
    println!("Hi. Mein Name ist {}!", mogree.red().bold().italic());
}

pub fn clear_screen() {
    // Clears the terminal with an ANSI escape code.
    // Works in UNIX and newer Windows terminals.
    println!("\x1Bc");
}

pub fn quit() -> bool {
    loop {
        println!("\n\nBeenden? (J/N):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "j" | "J" => {
                println!("Auf Wiedersehen!");
                return true;
            }
            "n" | "N" => return false,
            _ => {
                println!("Keine gültige Eingabe");
                println!("Bitte \"J\" für Ja eingeben oder \"N\" für Nein.\n");
            }
        }
    }
}
