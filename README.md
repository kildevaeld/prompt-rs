# Prompt

```rust

loop {
    let name = prompt::input("What is your name?")?;

    let choices = vec![
        "Pizza",
        "Burger",
        "Pasta"
    ];

    let food = prompt::select("What is your favorite food?", &choices)?;

    println!("Name: {}, food: {}", name, food);

    if confirm = prompt::confirm("Is it correct?")? {
        break;
    }

}



```