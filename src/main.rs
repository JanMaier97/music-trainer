use std::{collections::HashMap, io::{self, Write}};

use rand::{thread_rng, distributions::{uniform, Uniform}, Rng};

fn main() {
    let fiths = HashMap::from([
        ("C", "G"),
        ("D", "A"),
        ("E", "H"),
        ("F", "C"),
        ("G", "D"),
        ("A", "E"),
        ("H", "F#"),
    ]);

    let thirds = HashMap::from([
        ("C", "E"),
        ("D", "F#"),
        ("E", "G#"),
        ("F", "A"),
        ("G", "H"),
        ("A", "C#"),
        ("H", "D#"),
    ]);

    let major_chords = HashMap::from([
        ("C", "EG"),
        ("D", "F#A"),
        ("E", "G#H"),
        ("F", "AC"),
        ("G", "HD"),
        ("A", "C#E"),
        ("H", "D#F#"),
    ]);


    let thirds_level_1 = ["C", "D", "E", "F", "G", "A", "H"];
    let fiths_level_1 = ["C", "D", "E", "F", "G", "A", "H"];
    let major_chords_level_1 = ["C", "D", "E", "F", "G", "A", "H"];

    // play_guessing_game(&thirds, &thirds_level_1, "Name the third of");
    // play_guessing_game(&fiths, &fiths_level_1, "Name the fith of");
    play_guessing_game(&major_chords, &major_chords_level_1, "Major chord of");

}

fn play_guessing_game(answer_map: &HashMap<&str, &str>, questions: &[&str], prompt: &str) {
    let lenght = questions.len();
    let mut rng = thread_rng();
    let side = Uniform::new(0, lenght);

    for i in 0..20 {
        let index = rng.sample(side);
        let question = questions[index]; 
        let expected_answer = answer_map.get(question).expect("Question map not setup correctly");

        println!("Question {}",  i+1);

        loop {
            print!("{} {}: ", prompt, question);
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("error: unable to read user input");

            if answer.to_lowercase().trim() == expected_answer.to_lowercase() {
                println!("Correct!");
                break;
            } else {
                print!("Wrong, try again");
            }
            println!();
        }
    }
}
