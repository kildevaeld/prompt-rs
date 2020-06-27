extern crate prompt;
use failure::ResultExt;

fn main() {
    let mut form = prompt::Form::default();
    form.run(prompt::Confirm::new("Confirm this").default(false))
        .context("")
        .unwrap();

    let confirm = prompt::confirm("Confirm this?").unwrap();
    prompt::Input::new("name")
        .default("Rasmus")
        .required()
        .build()
        .run()
        .unwrap();
    prompt::passwd("password").unwrap();

    let choices = (1..200)
        .map(|m| format!("Choice {}", m))
        .collect::<Vec<_>>();

    let select = prompt::select("One choice:", &choices).unwrap();

    let radio = prompt::MultiSelect::new("Multiple choices:", &choices)
        .min(2)
        .max(5)
        .build()
        .run()
        .unwrap();

    println!("select {}", select);
}
