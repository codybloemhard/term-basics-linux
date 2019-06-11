extern crate term_basics_linux;
use term_basics_linux::tbl;

pub fn main(){
    tbl::print_cols_style("very nice", tbl::UserColour::Yellow, tbl::UserColour::Cyan, tbl::TextStyle::Bold);
    //test_test_chars();
    test_input_field();
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
    use term_basics_linux::tbl;
    for fg in tbl::UserColour::iterator(){
        for bg in tbl::UserColour::iterator(){
            tbl::set_colours(fg.clone(), bg.clone());
            println!("haha yes");
        }
    }
}

fn test_all_colours_styles(){
    use term_basics_linux::tbl;
    for sty in tbl::TextStyle::iterator(){
        for bg in tbl::UserColour::iterator(){
            for fg in tbl::UserColour::iterator(){
                tbl::set_style(sty.clone());
                tbl::set_colour(bg.clone(), tbl::FGBG::BG);
                tbl::set_colour(fg.clone(), tbl::FGBG::FG);
                println!("haha yes");
            }
        }
    }
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
