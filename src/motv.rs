use rand::Rng;
use std::{io, process};

pub fn mood_today() -> String {
    loop {
        println!("\n\nWas darf es sein?");
        println!("\nBitte die entsprechende Zahl eingeben");
        println!("[1]   => Motivation");
        println!("[2]   => Lustig");
        println!("[Q]   => Beenden");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "1" | "2" => return input.trim().to_string(),
            "q" | "Q" => {
                println!("Auf Wiedersehen!");
                process::exit(1);
            }
            _ => {
                println!("Keine gültige Eingabe");
                println!("Versuch es nochmal");
            }
        }
    }
}

pub fn have_motivation() {
    let motivations: Vec<_> = vec![
        "Um an die Quelle zu kommen, muss man gegen den Strom schwimmen. (Konfuzius)",
        "Glück lässt sich nicht erzwingen. Aber es mag hartnäckige Menschen.",
        "Alles, was du brauchst, um erfolgreich zu sein, hast du bereits in dir.",
        "Ohne Regen keinen Regenbogen -so einfach.",
        "Jetzt oder nie!"
    ];

    let len = motivations.len();
    let rnd_num = rand::thread_rng().gen_range(1..len);
    let mot =  &motivations[rnd_num];
    println!("{}\n", mot);
}

pub fn have_fun() {
    let funny: Vec<_> = vec![
        "Liebe Mathelehrer, schon mal dran gedacht, dass x vielleicht anonym bleiben will.",
        "Ich lasse mich zum Pandabären umschulen. Wenn ich faul rumliege und immer dicker werde, finden mich trotzdem alle niedlich.",
        "Gestern stand ich noch am Rande des Abgrunds, heute bin ich schon einen Schritt weiter!",
        "Ich mag Nashörner! Die sind wie Einhörner, nur dicker!",
        "/“Sie hören von meinem Anwalt!/” ist die Erwachsenen-Version von /“Das sag ich meiner Mama!/”",
        "Tut mir leid, ich habe weder die Geduld noch die Buntstifte es dir zu erklären."
    ];

    let len = funny.len();
    let rnd_num = rand::thread_rng().gen_range(1..len);
    let fun =  &funny[rnd_num];
    println!("{}\n", fun);
}
