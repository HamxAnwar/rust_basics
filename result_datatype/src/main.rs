// There might be cases in which we can have functions that fail in certain circumstances.
// A result is an enum datatype that results in either a successful data or an error data.
// It is used in scenarios where an action needs to be taken but may result in failure too.
// e.g. - copying a file, connecting to a website, etc.

/*

enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice is not present".to_owned()),
    }
}

fn print_choice(choice:&MenuChoice) {
    println!("Choice = {:?}", choice);
}

// The below function implements a ? which checks for a match case automatically in case of Result usage.
// This lets us use match cases without executing and writing multiple match blocks.
fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    println!("{:?}", &choice);
    Ok(())
}

fn main() {
    let choice = pick_choice("quit");
    println!("Choice Value = {:?}", choice);
}
