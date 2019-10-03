extern crate term_basics_linux;
use term_basics_linux as tbl;

pub fn main(){
    test_file_exist();
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

fn test_set_colour(){
    for i in tbl::UserColour::iterator(){
        tbl::set_colour(i.clone(), tbl::FGBG::FG);
        println!("haha yes");
    }
    tbl::set_colour(tbl::UserColour::Std, tbl::FGBG::FG);
    for i in tbl::UserColour::iterator(){
        tbl::set_colour(i.clone(), tbl::FGBG::BG);
        println!("haha yes");
    }
}

fn test_set_style(){
    for i in tbl::TextStyle::iterator(){
        tbl::set_colour(tbl::UserColour::Std, tbl::FGBG::FG);
        tbl::set_style(i.clone());
        println!("haha yes");
    }
}

fn test_set_colours(){
    use term_basics_linux as tbl;
    for fg in tbl::UserColour::iterator(){
        for bg in tbl::UserColour::iterator(){
            tbl::set_colours(fg.clone(), bg.clone());
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
                tbl::set_colour(bg.clone(), tbl::FGBG::BG);
                tbl::set_colour(fg.clone(), tbl::FGBG::FG);
                tbl::set_style(sty.clone());
                println!("haha yes");
            }
        }
    }
    //set style and colours independently
    for sty in tbl::TextStyle::iterator(){
        tbl::set_style(sty.clone());
        for bg in tbl::UserColour::iterator(){
            tbl::set_colour(bg.clone(), tbl::FGBG::BG);
            for fg in tbl::UserColour::iterator(){
                tbl::set_colour(fg.clone(), tbl::FGBG::FG);
                println!("haha yes");
            }
        }
    }
}

fn test_number_parse(){
    use term_basics_linux as tbl;
    let user_input = tbl::prompt("type your age: ");
    let age: Option<u8> = tbl::string_to_value(&user_input);
    if age.is_none() { println!("Invalid age!"); }
    else { println!("Your age: {}", age.unwrap()); }
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

fn test_get_home_string(){
    println!("{:?}", tbl::get_home_string());
}

fn test_get_home(){
    println!("{:?}", tbl::get_home());
}

fn test_file_exist(){
    use term_basics_linux as tbl;
    tbl::println(tbl::file_exists(tbl::get_home().unwrap().as_path()));
}
