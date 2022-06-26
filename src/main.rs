use mogree::*;

fn main() {
    loop {
        clear_screen();
        print_koala();
        let word: &str = "Hi";
        let colorword = colorit(word);
        println!("{colorword}");

        generate_symbol();

        if quit() {break;}
    }
}
