#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::io;
use std::io::Write; //flush stdout
use std::io::Read;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::cmp::{ max, min };

#[derive(Clone)]
pub enum MsgType {
    Normal,
    Error,
    Prompt,
    Highlight,
    Value,
}

fn set_colour(msgtype: MsgType){
    let colorcode = match msgtype {
        MsgType::Normal => "\x1B[32m",
        MsgType::Error => "\x1B[31m",
        MsgType::Prompt => "\x1B[36m",
        MsgType::Highlight => "\x1B[37m",
        MsgType::Value => "\x1B[33m",
    };
    print!("{}", colorcode);
}

pub fn print<T: std::fmt::Display>(msg: T){
    set_colour(MsgType::Normal);
    print!("{}", msg);
}

pub fn print_type<T: std::fmt::Display>(msg: T, msgtype: MsgType){
    set_colour(msgtype);
    print!("{}", msg);
}

pub fn print_error<T: std::fmt::Display>(pre: T, mid: T, pos: T){
    set_colour(MsgType::Error);
    print!("{}", pre);
    set_colour(MsgType::Highlight);
    print!("{}", mid);
    set_colour(MsgType::Error);
    print!("{}", pos);
}

pub fn println<T: std::fmt::Display>(msg: T){
    set_colour(MsgType::Normal);
    println!("{}", msg);
}

pub fn println_type<T: std::fmt::Display>(msg: T, msgtype: MsgType){
    set_colour(msgtype);
    println!("{}", msg);
}

pub fn println_error<T: std::fmt::Display>(pre: T, mid: T, pos: T){
    set_colour(MsgType::Error);
    print!("{}", pre);
    set_colour(MsgType::Highlight);
    print!("{}", mid);
    set_colour(MsgType::Error);
    println!("{}", pos);
}

pub fn getch() -> u8{
    //https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();
    return buffer[0];
}

pub fn test_chars(){
    loop {
        print!("{:?}\n", getch());
    }
}

fn custom_inp() -> String{
    fn charvec_to_string(vec: &Vec<char>) -> String{
        let mut string = String::new();
        for &ch in vec {
            string.push(ch);
        }
        return string;
    }
    fn typed_char(ch: u8, buff: &mut Vec<char>, astate: &mut u8, pos: &mut usize){
        let ch = ch as char;
        buff.insert(*pos, ch);
        if *pos != buff.len() - 1{
            for i in *pos..buff.len(){
                print!("{}", buff[i]);
            }
            for _ in  *pos..buff.len()-1{
                print!("{}", 8 as char);
            }
        }else{
            print!("{}", ch);
        }
        *astate = 0;
        *pos += 1;
    }
    let mut res = Vec::new();
    let mut arrow_state: u8 = 0;
    let mut hoen_state: u8 = 0;
    let mut pos = 0;

    set_colour(MsgType::Normal);
    loop {
        match getch(){
            10 => { print!("\n"); break; } //enter
            127 => {  //backspace
                if res.len() <= 0 { continue; }
                res.pop();
                for _ in 0..res.len() + 1 {
                    print!("{}", 8 as char);
                }
                let mut printres = res.clone();
                printres.push(' ');
                print(charvec_to_string(&printres));
                print!("{}", 8 as char);
                //print!("\x1B[1D"); //also works
                arrow_state = 0;
                pos = res.len();
            }
            27 => { //first char in arrow code and home/end code
                arrow_state = 1;
                hoen_state = 1;
            } 
            91 => { //2nd char in arrow code and home/end code
                if arrow_state == 1 { arrow_state = 2; }
                if hoen_state == 1 { hoen_state = 2; }
            }
            65 => { //up arrow 
                if arrow_state == 2 {}
                else { typed_char(65, &mut res, &mut arrow_state, &mut pos); }
            }
            66 => { //down arrow 
                if arrow_state == 2 {}
                else { typed_char(66, &mut res, &mut arrow_state, &mut pos); }
            }
            72 => { //home key
                if hoen_state != 2 { continue; }
                for _ in 0..pos {
                    print!("{}", 8 as char);
                }
                pos = 0;
                hoen_state = 0;
            }
            52 => { //end key 3e char
                if hoen_state == 2 { hoen_state = 3; }
            }
            126 => { //end key
                if hoen_state != 3 { continue; }
                for _ in pos..res.len() {
                    print!("\x1B[1C");
                }
                pos = res.len();
                hoen_state = 0;
            }
            67 => {  //right arrow
                if arrow_state == 2 {
                    if pos < res.len() { print!("\x1B[1C"); }
                    arrow_state = 0;
                    pos = min(pos + 1, res.len());
                }
                else { typed_char(67, &mut res, &mut arrow_state, &mut pos); }
            }
            68 => {  //left arrow
                if arrow_state == 2 {
                    if pos > 0 { print!("{}", 8 as char); }
                    arrow_state = 0;
                    pos = max(pos as i32 - 1, 0 as i32) as usize;
                }
                else { typed_char(68, &mut res, &mut arrow_state, &mut pos); }
            }
            x => { typed_char(x, &mut res, &mut arrow_state, &mut pos); }
        }
    }
    return charvec_to_string(&res);
}

pub fn prompt(msg : &str) -> String{
    print_type(msg, MsgType::Prompt);
    std::io::stdout().flush().expect("Error: stdout flush failed.");
    return custom_inp();
}

/*pub fn read_bool(msg: &str, inputs: &mut Option<VecDeque<astr::Astr>>) -> bool{
    let line;
    if inputs.is_none(){line = prompt(&msg);}
    else{
        let res = inputs.as_mut().unwrap().pop_front();
        if res.is_none(){line = prompt(&msg);}
        else {line = res.unwrap().tostring();}
    }
    match line.as_ref(){
        "y" => true,
        "ye" => true,
        "yes" => true,
        "ok" => true,
        "+" => true,
        "t" => true,
        "tr" => true,
        "tru" => true,
        "true" => true,
        _ => false,
    }
}*/
