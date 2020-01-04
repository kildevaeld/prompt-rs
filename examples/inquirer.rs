extern crate prompt;

fn main() {
    let confirm = prompt::confirm("Confirm this?").unwrap();
    prompt::input("name:").unwrap();
    prompt::passwd("password:").unwrap();

    let choices = (1..200)
        .map(|m| format!("Choice {}", m))
        .collect::<Vec<_>>();

    let select = prompt::select("One choice:", &choices).unwrap();

    let radio = prompt::MultiSelect::new("Multiple choices:", &choices).min(2).max(5).build().run().unwrap();

    println!("\n  select {}", select);
}
