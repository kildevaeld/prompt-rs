extern crate prompt;
use valid;

pub fn input() -> Result<(), Box<dyn std::error::Error>> {
    prompt::Input::new("Required input")
        .required()
        .build()
        .run()?;
    prompt::Input::new("Default input")
        .default("Rasmus")
        .build()
        .run()?;
    prompt::Input::new("Validated input")
        .validate(valid::MinLen(5))
        .build()
        .run()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input();

    let mut form = prompt::Form::default();
    form.run(prompt::Confirm::new("Confirm this").default(false))?;

    let confirm = prompt::confirm("Confirm this?")?;
    prompt::Input::new("name")
        //.default("Rasmus")
        .validate(valid::MinLen(6))
        .validate(valid::Parse::<f64>::new())
        .required()
        .build()
        .run()?;

    prompt::passwd("password")?;

    let choices = (1..200)
        .map(|m| format!("Choice {}", m))
        .collect::<Vec<_>>();

    let select = prompt::select("One choice:", &choices)?;

    let radio = prompt::MultiSelect::new("Multiple choices:", &choices)
        .validate(valid::MinLen(4))
        .validate(valid::MaxLen(5))
        .build()
        .run()?;

    println!("select {}", select);

    Ok(())
}
