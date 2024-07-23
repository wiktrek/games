use std::fmt::Debug;

/*
    1.split films into groups of 2
    2.make a select menu for each person
    3. make last select manu for films that were selected the most amount of times
*/
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
#[derive(Debug)]
struct Pick{
    one: String,
    two: String,
}
fn main() {
    let film = get_film();
    let films = film.split("\\n").collect::<Vec<&str>>();
    let split_films = split_films(films);
    display_films(split_films);
    // let decide = FuzzySelect::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Select film")
    //     .default(1)
    //     .items(&films)
    //     .interact()
    //     .unwrap();
}
fn display_films(vec: Vec<Pick>) {
    for v in vec {
        display_film(v);
    }
}
fn display_film(p: Pick) {
    if p.two == "stringisempty".to_string() {
        return
    }
    println!("{} - {:?}", p.one, p.two);
}
fn get_film() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter films separate them by writing \\n")
        .interact_text()
        .unwrap()
}
fn split_films(films: Vec<&str>) -> Vec<Pick> {
    let mut v: Vec<Pick> = vec![];
    for (i, f) in films.iter().enumerate() {
        if (i+1) % 2 == 0 && i != 0 {
            println!("{}", i);
            v.push(Pick {
                one: films[i - 1].to_string(),
                two: f.to_string(),
            })
        } else if i == films.len() -1 {
            println!("i ----- len, {}", i);
            v.push(Pick {
                one: f.to_string(),
                two: "stringisempty".to_string(),
            })
        }
    };
    v
}