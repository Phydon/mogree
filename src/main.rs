use mogree::*;

fn main() {
    loop {
        clear_screen();
        print_koala();

        generate_symbol();

        if quit() {break;}
    }
}
