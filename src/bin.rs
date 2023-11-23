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
fn test_input_field() {
    println!("{}", tbl::input_field());
}

#[allow(unused)]
fn test_input_field_scrollable() {
    let mut his = tbl::InputHistory::new(100);
    his.add("l");
    his.add("third");
    his.add("second");
    his.add("first");
    println!("{}", tbl::input_field_scrollable(&mut his));
}

#[allow(unused)]
fn test_prompt() {
    let name = tbl::prompt("type your name: ");
    print!("Your name: ");
    println!("{name}");
}

#[allow(unused)]
fn test_number_parse() {
    let user_input = tbl::prompt("type your age: ");
    let age: Option<u8> = tbl::string_to_value(&user_input);
    if let Some(agev) = age { println!("Your age: {}", agev); }
    else { println!("Invalid age!"); }
}

#[allow(unused)]
fn test_input_history_new() {
    let mut his = tbl::InputHistory::new(10);
    let _ = tbl::input_field_scrollable(&mut his);
}

#[allow(unused)]
fn test_input_history_add() {
    let mut his = tbl::InputHistory::new(2);
    his.add("0");
    his.add("1");
    his.add("2");
    //only "1" and "2" will remain, as 0 is removed.
    let _ = tbl::input_field_scrollable(&mut his);
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
fn test_prompt_scrollable() {
    let mut his = tbl::InputHistory::new(10);
    his.add("previously typed in name");
    let name = tbl::prompt_scrollable("type your name: ", &mut his);
    print!("Your name: ");
    println!("{name}");
}

#[allow(unused)]
fn test_input_field_custom() {
    // Hide the users password as it is typed in!
    let password = tbl::input_field_custom(
        &mut tbl::InputHistory::new(0), tbl::PromptChar::Substitude('*')
    );
    println!("{password}");
}

#[allow(unused)]
fn test_prompt_masked() {
    // Hide the users password as it is typed in!
    let password = tbl::prompt_masked("Enter password: ", '*');
    println!("{password}");
}

#[allow(unused)]
fn test_discard_newline_on_prompt_nexttime() {
    tbl::discard_newline_on_prompt_nexttime();
    let _ = tbl::prompt("enter your name: ");
    println!(" // your name");
}

#[allow(unused)]
fn test_use_newline_on_prompt() {
    tbl::discard_newline_on_prompt_nexttime(); // use somewhere
    tbl::use_newline_on_prompt(); // cancel somewhere else in code
    let _ = tbl::prompt("enter your name: ");
    println!(" // your name");
}

#[allow(unused)]
fn test_prompt_char() {
    println!("{}", tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Copy));
    println!("{}",
        tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Substitude('#'))
    );
    println!("{}", tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::None));
}

#[allow(unused)]
fn test_prompt_custom() {
    let mut his = tbl::InputHistory::new(2);
    his.add("hidden option 0");
    his.add("hidden option 1"); // provide options but the user can't see them.
    let x = tbl::prompt_custom("enter input:", &mut his, tbl::PromptChar::None);
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

#[allow(unused)]
fn test_input_field_scrollable_docu() {
    let mut history = tbl::InputHistory::new(100);
    let input0 = tbl::input_field_scrollable(&mut history);
    println!("You typed: {}", input0);
    let input1 = tbl::input_field_scrollable(&mut history);
    println!("You typed: {}", input1);
}
