use term_basics_linux as tbl;

pub fn main() {
    test_getch();
}

#[allow(unused)]
fn test_getch() {
    let x = tbl::getch();
    println!("{}", x as char);
}

#[allow(unused)]
fn test_test_chars() {
    tbl::test_chars();
}

#[allow(unused)]
fn test_input_field_simple() {
    print!("type your name: ");
    let name = tbl::input_field_simple(true);
    println!("your name: {name}");
}

#[allow(unused)]
fn test_number_parse() {
    print!("type your age: ");
    let user_input = tbl::input_field_simple(true);
    let age: Option<u8> = tbl::string_to_value(&user_input);
    if let Some(agev) = age { println!("Your age: {}", agev); }
    else { println!("Invalid age!"); }
}

#[allow(unused)]
fn test_input_history_new() {
    let mut his = tbl::InputHistory::new(10);
    let _ = tbl::input_field_history(&mut his, true);
}

#[allow(unused)]
fn test_input_history_add() {
    let mut his = tbl::InputHistory::new(2);
    his.add("0");
    his.add("1");
    his.add("2");
    //only "1" and "2" will remain, as 0 is removed.
    let _ = tbl::input_field_history(&mut his, true);
}

#[allow(unused)]
fn test_input_history_get_index() {
    let mut his = tbl::InputHistory::new(3);
    his.add("0");
    his.add("1");
    his.add("2");
    println!("at -2: {:?}", his.get_index(-2));
    println!("at -1: {:?}", his.get_index(-1));
    println!("at  0: {:?}", his.get_index(0));
    println!("at  1: {:?}", his.get_index(1));
    println!("at  2: {:?}", his.get_index(2));
    println!("at  3: {:?}", his.get_index(3));
    println!("at  4: {:?}", his.get_index(4));
}

#[allow(unused)]
fn test_input_field_scrollable() {
    let mut his = tbl::InputHistory::new(10);
    his.add("previously typed in name");
    print!("type your name: ");
    let name = tbl::input_field_history(&mut his, true);
    println!("your name: {name}");
}

#[allow(unused)]
fn test_prompt_char() {
    println!("{}", tbl::input_field(&mut tbl::InputHistory::new(0), tbl::PromptChar::Copy, true));
    println!("{}",
        tbl::input_field(&mut tbl::InputHistory::new(0), tbl::PromptChar::Substitute('*'), true)
    );
    println!("{}", tbl::input_field(&mut tbl::InputHistory::new(0), tbl::PromptChar::None, true));
}

#[allow(unused)]
fn test_prompt_custom() {
    let mut his = tbl::InputHistory::new(2);
    his.add("hidden option 0");
    his.add("hidden option 1"); // provide options but the user can't see them.
    print!("enter input: ");
    let x = tbl::input_field_history(&mut his, true);
    println!("{x}");
}

// documentation integration tests, that are not included above

#[allow(unused)]
fn test_getch_docu() {
    // print user input until spacebar is pressed
    loop{
        let x = tbl::getch();
        if x == 32 { break; }
        print!("{}", x as char);
    }
}

