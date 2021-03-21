extern crate term_basics_linux;
use term_basics_linux as tbl;

pub fn main(){
    test_prompt();
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

fn test_input_field_scrollable(){
    let mut his = tbl::InputHistory::new(100);
    his.add(&"l".to_string());
    his.add(&"third".to_string());
    his.add(&"second".to_string());
    his.add(&"first".to_string());
    tbl::println(tbl::input_field_scrollable(&mut his));
}

fn test_prompt(){
    let name = tbl::prompt("type your name: ");
    tbl::print("Your name: ");
    tbl::println(name);
}

fn test_set_colour(){
    for i in tbl::UserColour::iterator(){
        tbl::set_colour(*i, tbl::FGBG::FG);
        println!("haha yes");
    }
    tbl::set_colour(tbl::UserColour::Std, tbl::FGBG::FG);
    for i in tbl::UserColour::iterator(){
        tbl::set_colour(*i, tbl::FGBG::BG);
        println!("haha yes");
    }
}

fn test_set_style(){
    for i in tbl::TextStyle::iterator(){
        tbl::set_colour(tbl::UserColour::Std, tbl::FGBG::FG);
        tbl::set_style(*i);
        println!("haha yes");
    }
}

fn test_set_colours(){
    use term_basics_linux as tbl;
    for fg in tbl::UserColour::iterator(){
        for bg in tbl::UserColour::iterator(){
            tbl::set_colours(*fg, *bg);
            println!("haha yes");
        }
    }
}

fn test_all_colours_styles(){
    use term_basics_linux as tbl;
    //can be set in any ordering
    for sty in tbl::TextStyle::iterator(){
        for bg in tbl::UserColour::iterator(){
            for fg in tbl::UserColour::iterator(){
                tbl::set_colour(*bg, tbl::FGBG::BG);
                tbl::set_colour(*fg, tbl::FGBG::FG);
                tbl::set_style(*sty);
                println!("haha yes");
            }
        }
    }
    //set style and colours independently
    for sty in tbl::TextStyle::iterator(){
        tbl::set_style(*sty);
        for bg in tbl::UserColour::iterator(){
            tbl::set_colour(*bg, tbl::FGBG::BG);
            for fg in tbl::UserColour::iterator(){
                tbl::set_colour(*fg, tbl::FGBG::FG);
                println!("haha yes");
            }
        }
    }
}

fn test_number_parse(){
    use term_basics_linux as tbl;
    let user_input = tbl::prompt("type your age: ");
    let age: Option<u8> = tbl::string_to_value(&user_input);
    if let Some(agev) = age { println!("Your age: {}", agev); }
    else { println!("Invalid age!"); }
}

fn test_input_history_new(){
    use term_basics_linux as tbl;
    let mut his = tbl::InputHistory::new(10);
    let _ = tbl::input_field_scrollable(&mut his);
}

fn test_input_history_add(){
    use term_basics_linux as tbl;
    let mut his = tbl::InputHistory::new(2);
    his.add(&"0".to_string());
    his.add(&"1".to_string());
    his.add(&"2".to_string());
    //only "1" and "2" will remain, as 0 is removed.
    let _ = tbl::input_field_scrollable(&mut his);
}

fn test_input_history_get_index(){
    use term_basics_linux as tbl;
    let mut his = tbl::InputHistory::new(3);
    his.add(&"0".to_string());
    his.add(&"1".to_string());
    his.add(&"2".to_string());
    println!("at -2: {:?}", his.get_index(-2));
    println!("at -1: {:?}", his.get_index(-1));
    println!("at  0: {:?}", his.get_index(0));
    println!("at  1: {:?}", his.get_index(1));
    println!("at  2: {:?}", his.get_index(2));
    println!("at  3: {:?}", his.get_index(3));
    println!("at  4: {:?}", his.get_index(4));
}

fn test_prompt_scrollable(){
    use term_basics_linux as tbl;
    let mut his = tbl::InputHistory::new(10);
    his.add(&"previously typed in name".to_string());
    let name = tbl::prompt_scrollable("type your name: ", &mut his);
    tbl::print("Your name: ");
    tbl::println(name);
}

fn test_input_field_custom(){
    use term_basics_linux as tbl;
    let password = tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Substitude('*')); //Hide the users password as it is typed in!
    tbl::println_style(password, tbl::TextStyle::Bold); // THAN PRINT IT OUT
}

fn test_prompt_masked(){
    use term_basics_linux as tbl;
    let password = tbl::prompt_masked("Enter password: ", '*'); // Hide the users password as it is typed in!
    tbl::println_style(password, tbl::TextStyle::Blink); // show it to the world with some extra spice.
}

fn test_persistant(){
    use term_basics_linux as tbl;
    tbl::println_col("cyan", tbl::UserColour::Cyan);
    println!("after cyan");
    tbl::set_colour(tbl::UserColour::Red, tbl::FGBG::FG);
    tbl::println("red");
    tbl::use_colour(tbl::UserColour::Yellow, tbl::FGBG::FG);
    tbl::println("yellow");
    tbl::restore_colour(tbl::FGBG::FG);
    tbl::println("red");
    println!("still red");
}

fn test_use_colour(){
    use term_basics_linux as tbl;
    tbl::use_colour(tbl::UserColour::Blue, tbl::FGBG::FG);
    tbl::println("henlo frens");
}

fn test_restore_colour(){
    use term_basics_linux as tbl;
    tbl::set_colour(tbl::UserColour::Red, tbl::FGBG::FG);
    tbl::println("this is red");
    tbl::use_colour(tbl::UserColour::Green, tbl::FGBG::FG);
    tbl::println("this is green");
    tbl::restore_colour(tbl::FGBG::FG);
    tbl::println("this is red again");
}

fn test_use_colours(){
    use term_basics_linux as tbl;
    for fg in tbl::UserColour::iterator(){
        for bg in tbl::UserColour::iterator(){
            tbl::use_colours(*fg, *bg);
            println!("haha yes");
        }
    }
}

fn test_restore_colours(){
    use term_basics_linux as tbl;
    tbl::set_colours(tbl::UserColour::Green, tbl::UserColour::Magenta);
    tbl::println("cool and good");
    tbl::use_colours(tbl::UserColour::Red, tbl::UserColour::Black);
    tbl::println("warm and bad");
    tbl::restore_colours();
    tbl::println("cool and good again");
}

fn test_use_style(){
    use term_basics_linux as tbl;
    for i in tbl::TextStyle::iterator(){
        tbl::use_style(*i);
        println!("haha yes");
    }
}

fn test_restore_style(){
    use term_basics_linux as tbl;
    tbl::set_style(tbl::TextStyle::Bold);
    tbl::println("this is bold");
    tbl::use_style(tbl::TextStyle::Crossed);
    tbl::println("this is crossed");
    tbl::restore_style();
    tbl::println("this is bold again");
}

fn test_reset_colours(){
    use term_basics_linux as tbl;
    tbl::set_colours(tbl::UserColour::Magenta, tbl::UserColour::Yellow);
    tbl::set_style(tbl::TextStyle::Underlined);
    tbl::println("i am magenta on yellow as well as underlined");
    tbl::reset_colours();
    tbl::println("i am underlined but have standard colours")
}

fn test_reset_style(){
    use term_basics_linux as tbl;
    tbl::set_colours(tbl::UserColour::Cyan, tbl::UserColour::Red);
    tbl::set_style(tbl::TextStyle::Blink);
    tbl::println("im am cyan on red and blinking");
    tbl::reset_style();
    tbl::println("i am still cyan on red but im am not blinking");
}

fn test_discard_newline_on_prompt_nexttime(){
    use term_basics_linux as tbl;
    tbl::discard_newline_on_prompt_nexttime();
    let _ = tbl::prompt("enter your name: ");
    tbl::println(" // your name");
}

fn test_use_newline_on_prompt(){
    use term_basics_linux as tbl;
    tbl::discard_newline_on_prompt_nexttime();//use somewhere
    tbl::use_newline_on_prompt();//cancel somewhere else in code
    let _ = tbl::prompt("enter your name: ");
    tbl::println(" // your name");
}

fn test_prompt_char(){
    use term_basics_linux as tbl;
    tbl::println(tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Copy));
    tbl::println(tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Substitude('#')));
    tbl::println(tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::None));
}

fn test_prompt_custom(){
    use term_basics_linux as tbl;
    let mut his = tbl::InputHistory::new(2);
    his.add(&"hidden option 0".to_string());
    his.add(&"hidden option 1".to_string());//provide options but the user can't see them.
    let x = tbl::prompt_custom("enter input:", &mut his, tbl::PromptChar::None);
    tbl::println(x);
}

fn test_println_cols_style(){
    use term_basics_linux as tbl;
    tbl::println_cols_style("test", tbl::UserColour::Red, tbl::UserColour::Yellow, tbl::TextStyle::Crossed);
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

fn test_input_field_scrollable_docu(){
    use term_basics_linux as tbl;
    let mut history = tbl::InputHistory::new(100);
    let input0 = tbl::input_field_scrollable(&mut history);
    println!("You typed: {}", input0);
    let input1 = tbl::input_field_scrollable(&mut history);
    println!("You typed: {}", input1);
}
