use mogree::*;
pub mod motv;
use crate::motv::*;

fn main() {
    loop {
        clear_screen();
        print_koala();

        generate_symbol();

        let mood = mood_today();
        match mood.trim() {
            "1" => have_motivation(),
            "2" => have_fun(),
            _ => panic!("Keine Eingabe!"),
        };

        if quit() {break;}
    }
}
