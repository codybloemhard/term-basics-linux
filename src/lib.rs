//! # Example
//!
//! ```
//! use term_basics_linux as tbl;
//! print!("typ your name: ");
//! let name = tbl::input_field_simple(true);
//! print!("Your name: {name}");
//! ```

use std::{
    io::{ self, Write, Read },
    cmp::{ max, min },
    collections::VecDeque,
};

use termios::{ Termios, TCSANOW, ECHO, ICANON, tcsetattr };

/// Reads a char from keyboard input.
/// Returns the first byte, does not wait for the user to press enter.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// println!("Press any key...");
/// let anykey = tbl::getch();
/// ```
pub fn getch() -> u8 {
    // https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios;
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &new_termios).unwrap();
    let stdout = io::stdout();
    let reader = io::stdin();
    let mut buffer = [0; 1];
    stdout.lock().flush().unwrap();
    reader.lock().read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
    buffer[0]
}

/// Prints the result of getch as u8, infinite loop. Can be used for testing.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// let user_input = tbl::test_chars();
/// ```
pub fn test_chars() {
    loop {
        println!("{:?}", getch());
    }
}

/// A struct that holds inputs available for the user to scroll through.
pub struct InputList {
    ilist: VecDeque<String>,
    maxlen: usize,
}

impl InputList {
    /// Make a new InputList with a certain maximum capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use term_basics_linux as tbl;
    /// let mut his = tbl::InputList::new(10);
    /// let x = tbl::input_field_scrollable(&mut his, true);
    /// ```
    pub fn new(maxlen: usize) -> Self {
        Self{
            ilist: VecDeque::new(),
            maxlen
        }
    }

    fn trim(&mut self){
        while self.ilist.len() > self.maxlen {
            self.ilist.pop_front();
        }
    }

    /// Adds an element to the list.
    /// It will delete items if it's length would grow past the max length.
    /// the oldest items will be removed.
    ///
    /// # Example
    ///
    /// ```
    /// use term_basics_linux as tbl;
    /// let mut his = tbl::InputList::new(2);
    /// his.add(&"0".to_string());
    /// his.add(&"1".to_string());
    /// his.add(&"2".to_string());
    /// // only "1" and "2" will remain, as 0 is removed.
    /// let _ = tbl::input_field_scrollable(&mut his, true);
    /// ```
    pub fn add(&mut self, string: &str) {
        self.ilist.push_back(string.to_string());
        self.trim();
    }

    /// Returns the an Option of String, the element at the given index.
    /// The index wraps and you can query for negative indices as well as indices above the maximum length.
    ///
    /// # Example
    ///
    /// ```
    /// use term_basics_linux as tbl;
    /// let mut his = tbl::InputList::new(3);
    /// his.add(&"0".to_string());
    /// his.add(&"1".to_string());
    /// his.add(&"2".to_string());
    /// println!("at -2: {:?}", his.get_index(-2)); // "1"
    /// println!("at -1: {:?}", his.get_index(-1)); // "2"
    /// println!("at  0: {:?}", his.get_index(0));  // "0"
    /// println!("at  1: {:?}", his.get_index(1));  // "1"
    /// println!("at  2: {:?}", his.get_index(2));  // "2"
    /// println!("at  3: {:?}", his.get_index(3));  // "0"
    /// println!("at  4: {:?}", his.get_index(4));  // "1"
    /// ```
    pub fn get_index(&self, mut index: i32) -> Option<&String> {
        if !self.ilist.is_empty(){
            index %= self.ilist.len() as i32;
        }
        if index < 0{
            index += self.maxlen as i32;
        }
        self.ilist.get(index as usize)
    }
}

/// What kind of character the input field will print.
/// ```Copy``` will just print what the user types in.
/// ```Substitute(char)``` will print that character.
/// ```None``` will not print anything at all.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// println!("{}", tbl::input_field(&mut tbl::InputList::new(0), tbl::PrintChar::Copy, true));
/// println!("{}", tbl::input_field(&mut tbl::InputList::new(0), tbl::PrintChar::Substitute('#'), true));
/// println!("{}", tbl::input_field(&mut tbl::InputList::new(0), tbl::PrintChar::None, true));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PrintChar {
    Copy,
    Substitute(char),
    None,
}

pub fn input_field(ilist: &mut InputList, pc: PrintChar, newline: bool) -> String {
    fn charvec_to_string(vec: &[char]) -> String {
        let mut string = String::new();
        for &ch in vec {
            string.push(ch);
        }
        string
    }
    fn typed_char(
        ch: u8,
        buff: &mut Vec<char>,
        gstate: &mut u8,
        hstate: &mut u8,
        pos: &mut usize,
        pc: PrintChar
    ){
        let ch = ch as char;
        buff.insert(*pos, ch);
        if pc != PrintChar::None {
            if *pos != buff.len() - 1 {
                for item in buff.iter().skip(*pos) {
                    put_char(*item, pc);
                }
                go_back(*pos, buff.len() - 1, pc);
            } else {
                put_char(ch, pc);
            }
        }
        *hstate = 0;
        *gstate = 0;
        *pos += 1;
    }
    fn delete_all(buff: &mut Vec<char>, pc: PrintChar) -> usize {
        if pc == PrintChar::None {
            buff.clear();
            return 0;
        }
        let len = buff.len();
        go_back(0, len, pc);
        buff.clear();
        len
    }
    fn feed_into_buffer(buff: &mut Vec<char>, string: &str) {
        for ch in string.chars() {
            buff.push(ch);
        }
    }
    fn write_all(buff: &[char], pc: PrintChar) {
        for item in buff.iter() {
            put_char(*item, pc);
        }
    }
    fn scroll_action(
        res: &mut Vec<char>,
        pos: &mut usize,
        ilist: &InputList,
        his_index: i32,
        pc: PrintChar
    ){
        let val = ilist.get_index(his_index);
        if let Some(valv) = val {
            let old_len = delete_all(res, pc);
            feed_into_buffer(res, valv);
            *pos = res.len();
            if pc == PrintChar::None { return; }
            write_all(res, pc);
            let diff = old_len as i32 - res.len() as i32;
            if diff <= 0 { return; }
            for _ in 0..diff {
                print!(" ");
            }
            for _ in 0..diff {
                print!("{}", 8 as char);
            }
        }
    }
    fn delete(res: &mut Vec<char>, pos: &mut usize, gstate: &mut u8, pc: PrintChar) {
        if res.is_empty() { return; }
        if *pos >= res.len() - 1 { return; }
        res.remove(*pos);
        for item in res.iter().skip(*pos) {
            put_char(*item, pc);
        }
        if pc != PrintChar::None { print!(" "); }
        go_back(*pos, res.len() + 1, pc);
        *gstate = 0;
    }
    fn end(res: &mut [char], pos: &mut usize, hoen_state: &mut u8, pc: PrintChar) {
        if pc != PrintChar::None {
            for _ in *pos..res.len() {
                print!("\x1B[1C");
            }
        }
        *hoen_state = 0;
        *pos = res.len();
    }
    fn put_char(ch: char, pc: PrintChar) {
        match pc {
            PrintChar::Copy => print!("{}", ch),
            PrintChar::Substitute(sch) => print!("{}", sch),
            PrintChar::None => {},
        };
    }
    fn go_back(start: usize, end: usize, pc: PrintChar) {
        if pc == PrintChar::None { return; }
        for _ in  start..end {
            print!("{}", 8 as char);
        }
    }

    let mut res = Vec::new();
    let mut gstate: u8 = 0;
    let mut hoen_state: u8 = 0;
    let mut pos = 0;
    let mut his_index: i32 = 0;

    flush().expect("term_basics_linux: Error: stdout flush failed.");
    loop {
        let mut x = getch();
        if x == 8 { x = 127; } // shift + backspace
        match x {
            10 => { // enter
                if newline {
                    println!();
                }
                break;
            }
            127 => { // backspace
                if res.is_empty() { continue; }
                if pos == 0 { continue; }
                res.remove(pos - 1);
                if pc != PrintChar::None {
                    print!("{}", 8 as char);
                    //print!("\x1B[1D"); // also works
                    for item in res.iter().skip(pos - 1){
                        put_char(*item, pc);
                    }
                    print!(" ");
                    go_back(pos - 1, res.len() + 1, pc);
                }
                pos -= 1;
                gstate = 0;
            }
            27 => { // first char in arrow code and home/end code and other char combos
                gstate = 1;
                hoen_state = 1;
            }
            91 => { // 2nd char in arrow code and home/end code and other char combos
                if gstate == 1 { gstate = 2; }
                if hoen_state == 1 { hoen_state = 2; }
                if gstate == 2 || hoen_state == 2 { continue; }
                typed_char(91, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
            }
            65 => { // up arrow
                if gstate == 2 {
                    gstate = 0;
                    his_index += 1;
                    scroll_action(&mut res, &mut pos, ilist, his_index, pc);
                }
                else { typed_char(65, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            66 => { // down arrow
                if gstate == 2 {
                    gstate = 0;
                    his_index -= 1;
                    scroll_action(&mut res, &mut pos, ilist, his_index, pc);
                }
                else { typed_char(66, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            72 => { // home key
                if hoen_state != 2 {
                    typed_char(72, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                    continue;
                }
                go_back(0, pos, pc);
                pos = 0;
                hoen_state = 0;
            }
            51 => {
                if gstate != 2 {
                    typed_char(51, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                }
                else {
                    gstate = 3;
                }
            }
            52 => { // end key 3e char
                if hoen_state == 2 { hoen_state = 3; }
                else { typed_char(52, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            70 =>{ // end(27-91-70)
                if hoen_state == 2 {
                    end(&mut res, &mut pos, &mut hoen_state, pc);
                }
                else{
                    typed_char(70, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                }
            }
            126 => { // end(27-91-52-126) or delete(27-91-51-126)
                if hoen_state == 3 { // end
                    end(&mut res, &mut pos, &mut hoen_state, pc);
                }
                else if gstate >= 2{ // delete
                    delete(&mut res, &mut pos, &mut gstate, pc);
                }
                else {
                    typed_char(126, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                }
            }
            80 => { // delete key with code 27-91-80
                if gstate != 2 {
                    typed_char(80, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                    continue;
                }
                delete(&mut res, &mut pos, &mut gstate, pc);
            }
            67 => { // right arrow
                if gstate == 2 {
                    let old_pos = pos;
                    pos = min(pos + 1, res.len());
                    gstate = 0;
                    if pc == PrintChar::None { continue; }
                    // if pos < res.len() { print!("\x1B[1C"); }
                    if pos > old_pos { print!("\x1B[1C"); }
                }
                else { typed_char(67, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            68 => { // left arrow
                if gstate == 2 {
                    let old_pos = pos;
                    pos = max(pos as i32 - 1, 0_i32) as usize;
                    gstate = 0;
                    if pc == PrintChar::None { continue; }
                    // if pos > 0 { print!("{}", 8 as char); }
                    if pos < old_pos { print!("{}", 8 as char); }
                }
                else { typed_char(68, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            x => { typed_char(x, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
        }
    }
    let string = charvec_to_string(&res);
    ilist.add(&string);
    string
}

pub fn string_to_bool(string: &str) -> bool {
    matches!(string, "y" | "ye" | "yes" | "ok" | "+" | "t" | "tr" | "tru" | "true")
}

/// Small helper to parse string to a value
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// print!("type your age: ");
/// let user_input = tbl::input_field_simple(true);
/// let age: Option<u8> = tbl::string_to_value(&user_input);
/// if age.is_none() { println!("Invalid age!"); }
/// else { println!("Your age: {}", age.unwrap()); }
/// ```
/// Uses ```string.parse::<T>();```
pub fn string_to_value<T: std::str::FromStr>(string: &str) -> Option<T> {
    let res = string.parse::<T>();
    if res.is_err() { return Option::None; }
    res.ok()
}

/// Flushes stdout.
/// When you do print! or term-basics-linux equivalent, it will not print immediately.
/// flush() will make sure everything is printed first, before you do something else.
/// Input fields flush themselves before the query for input.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// print!("type: ");
/// tbl::flush().expect("oh no");
/// ```
///
pub fn flush() -> io::Result<()> {
    std::io::stdout().flush()
}

/// Prints a message to the user.
/// The user can  type its input next to the message on the same line.
/// It will return the user input after the user pressed enter.
/// It uses term_basics_linux::input_field and supports the same operations.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// print!("type your name: ");
/// let name = tbl::input_field_simple(true);
/// print!("Your name: ");
/// println!("{name}");
/// ```
pub fn input_field_simple(newline: bool) -> String {
    input_field(&mut InputList::new(0), PrintChar::Copy, newline)
}

pub fn input_field_scrollable(ilist: &mut InputList, newline: bool) -> String {
    input_field(ilist, PrintChar::Copy, newline)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn string_to_int0(){
        let t: Option<u32> = super::string_to_value(&String::from("12981398"));
        assert_eq!(t, Option::Some(12981398));
    }

    #[test]
    fn string_to_int1(){
        let t: Option<i32> = super::string_to_value(&String::from("-1234"));
        assert_eq!(t, Option::Some(-1234));
    }

    #[test]
    fn string_to_int2(){
        let t: Option<u8> = super::string_to_value(&String::from("70000"));
        assert_eq!(t, Option::None);
    }

    #[test]
    fn string_to_int3(){
        let t: Option<i32> = super::string_to_value(&String::from("23ohno23"));
        assert_eq!(t, Option::None);
    }

    #[test]
    fn string_to_float0(){
        let t: Option<f32> = super::string_to_value(&String::from("34.5"));
        assert_eq!(t, Option::Some(34.5));
    }

    #[test]
    fn string_to_float1(){
        let t: Option<f64> = super::string_to_value(&String::from("-0.00000000000001"));
        assert_eq!(t, Option::Some(-0.00000000000001));
    }

    #[test]
    fn string_to_bool0(){
        assert!(super::string_to_bool(&String::from("yes")));
    }

    #[test]
    fn string_to_bool1(){
        let t: Option<bool> = super::string_to_value(&String::from("true"));
        assert_eq!(t, Option::Some(true));
    }

    #[test]
    fn string_to_bool2(){
        let t: Option<bool> = super::string_to_value(&String::from("false"));
        assert_eq!(t, Option::Some(false));
    }
}

