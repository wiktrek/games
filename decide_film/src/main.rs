/*
    1.split films into groups of 2
    2.make a select menu for each person
    3. make last select manu for films that were selected the most amount of times
*/
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
fn main() {
    let film = get_film();
    let films = film.split("\\n").collect::<Vec<&str>>();
    let decide = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select film")
        .default(1)
        .items(&films)
        .interact()
        .unwrap();
}
fn get_film() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter films separate them by writing \\n")
        .interact_text()
        .unwrap()
}