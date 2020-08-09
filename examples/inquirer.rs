extern crate prompt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut form = prompt::Form::default();
    form.run(prompt::Confirm::new("Confirm this").default(false))?;

    let confirm = prompt::confirm("Confirm this?")?;
    prompt::Input::new("name")
        //.default("Rasmus")
        .validate(prompt::validation::MinLen(6))
        .validate(prompt::validation::Parse::<f64>::new())
        .required()
        .build()
        .run()?;

    prompt::passwd("password")?;

    let choices = (1..200)
        .map(|m| format!("Choice {}", m))
        .collect::<Vec<_>>();

    let select = prompt::select("One choice:", &choices)?;

    let radio = prompt::MultiSelect::new("Multiple choices:", &choices)
        // .min(2)
        // .max(5)
        .validate(prompt::validation::MinLen(4))
        .validate(prompt::validation::MaxLen(5))
        .build()
        .run()?;

    println!("select {}", select);

    Ok(())
}
