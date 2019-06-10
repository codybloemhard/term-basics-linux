extern crate term_basics_linux;
use term_basics_linux::tbl;

pub fn main(){
    use term_basics_linux::tbl::getch;
    //print user input until spacebar is pressed
    loop{
        let x = getch();
        if x == 32 { break; }
        print!("{}", x as char);
    }
}

fn test_getch(){
    let x = tbl::getch();
    tbl::println(x as char);
}

fn test_test_chars(){
    tbl::test_chars();
}

fn test_input_field(){
    tbl::println(tbl::input_field());
}

fn test_prompt(){
    let name = tbl::prompt("type your name: ");
    tbl::print("Your name: ");
    tbl::println(name);
}

//documentation integration tests, that are not included above

fn test_getch_docu(){
    //print user input until spacebar is pressed
    loop{
        let x = tbl::getch();
        if x == 32 { break; }
        print!("{}", x as char);
    }
}