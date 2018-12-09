extern crate prompt;

fn main() {
    prompt::input("name:").unwrap();
    prompt::passwd("password:").unwrap();

    let choices = (0..20).map(|m| format!("Choice {}", m)).collect::<Vec<_>>();

    let select = prompt::select("vælg:", &choices).unwrap();

    println!("\n  select {}", select);
}
