// ? Result<T, E>
// ? T : placeholder for any type you like
// ? E : placeholder for any type as well, but for error
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn print_choice(choice: &MenuChoice) {
    println!("choices = {:?}", choice)
}

fn pick_choice(input: &str) -> Result<(), String> {
    // ? ? (question mark) : automatically return and break if Error occurs
    let choice: MenuChoice = get_choice(input)?;

    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice = get_choice("quit");
    print_choice(&MenuChoice::MainMenu);

    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }

    let choice2 = pick_choice("leave");
    println!("choice value = {:?}", choice2)
}
