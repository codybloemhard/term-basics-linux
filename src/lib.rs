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
        assert_eq!(super::string_to_bool(&String::from("yes")), true);
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
/// # Examples
/// ```
/// use term_basics_linux as tbl;
/// for sty in tbl::TextStyle::iterator(){
///     for bg in tbl::UserColour::iterator(){
///         for fg in tbl::UserColour::iterator(){
///             tbl::println_cols_style("cool and good", fg, bg, sty);
///         }
///     }
/// }
/// ```
/// ```
/// let name = tbl::prompt("type your name: ");
/// tbl::print("Your name: ");
/// tbl::println(name);
/// ```
use std::io;
use std::io::Write; //flush stdout
use std::io::Read;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::cmp::{ max, min };
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use std::slice::Iter;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use std::collections::VecDeque;

static FG_COL: AtomicU8 = AtomicU8::new(9);
static BG_COL: AtomicU8 = AtomicU8::new(9);

/// Resets foreground colour, background colour and text style.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::reset_all();
/// ```
pub fn reset_all(){
    print!("\x1B[00m");
}

/// Colours available. The user has defined the exact values of 
/// these colours for there TTY or emulator.
#[derive(PartialEq,Eq,Clone,ToPrimitive)]
pub enum UserColour {
    Std     = 9,
    Black   = 0,
    Red     = 1,
    Green   = 2,
    Yellow  = 3,
    Blue    = 4,
    Magenta = 5,
    Cyan    = 6,
    Grey    = 7,
}
/// Iterate over all colours in the enum
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// for col in tbl::UserColour::iterator(){
///     //use col
/// }
/// ```
impl UserColour {
    pub fn iterator() -> Iter<'static, Self> {
        static ARR: [UserColour; 9] = [
            UserColour::Std, 
            UserColour::Black,
            UserColour::Red,
            UserColour::Green,
            UserColour::Yellow,
            UserColour::Blue,
            UserColour::Magenta,
            UserColour::Cyan,
            UserColour::Grey];
        ARR.iter()
    }
}
/// All styles that do not alter fg or bg colours.
#[derive(PartialEq,Eq,Clone,ToPrimitive)]
pub enum TextStyle {
    Std         = 0,
    Bold        = 1,
    Faint       = 2,
    Italic      = 3,
    Underlined  = 4,
    Blink       = 5,
    Hidden      = 8,
    Crossed     = 9,
}
/// Iterate over all styles.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// for sty in tbl::TextStyle::iterator(){
///     //use sty
/// }
/// ```
impl TextStyle {
    pub fn iterator() -> Iter<'static, Self> {
        static ARR: [TextStyle; 8] = [
            TextStyle::Std,
            TextStyle::Bold,
            TextStyle::Faint,
            TextStyle::Italic,
            TextStyle::Underlined,
            TextStyle::Blink,
            TextStyle::Hidden,
            TextStyle::Crossed];
        ARR.iter()
    }
}
/// To specify if you set the foreground or background colour.
#[derive(PartialEq,Eq)]
pub enum FGBG { FG, BG }
/// Sets the colour of the text printed after this call.
/// It will print linux colour escape characters to std out.
/// 
/// # Examples
/// 
/// ```
/// use term_basics_linux as tbl;
/// for i in tbl::UserColour::iterator(){
///     tbl::set_colour(i.clone(), tbl::FGBG::FG);
///     println!("haha yes");
/// }
/// ```
/// ```
/// use term_basics_linux as tbl;
/// for i in tbl::UserColour::iterator(){
///     tbl::set_colour(i.clone(), tbl::FGBG::BG);
///     println!("haha yes");
/// }
/// ```
pub fn set_colour(col: UserColour, fgbg: FGBG){
    let _id = ToPrimitive::to_u8(&col);
    let mut id = 0;
    if let Some(_idv) = _id { id = _idv; }
    let mut colorcode = String::from("\x1B[");
    if fgbg == FGBG::FG {
        colorcode.push('3');
        FG_COL.store(id, Ordering::Relaxed);
    }else{
        colorcode.push('4');
        BG_COL.store(id, Ordering::Relaxed);
    }
    colorcode.push_str(&format!("{}", id));
    colorcode.push_str("m");
    print!("{}", colorcode);
}
/// Sets both foreground and background colours
/// It will print linux colour escape characters to std out.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// for fg in tbl::UserColour::iterator(){
///     for bg in tbl::UserColour::iterator(){
///         tbl::set_colours(fg.clone(), bg.clone());
///         println!("haha yes");
///     }
/// }
/// ```
pub fn set_colours(fg: UserColour, bg: UserColour){
    set_colour(fg, FGBG::FG);
    set_colour(bg, FGBG::BG);
}
/// Sets the style of the text printed after this call.
/// It will print linux colour escape characters to std out.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// for i in tbl::TextStyle::iterator(){
///     tbl::set_style(i.clone());
///     println!("haha yes");
/// }
/// ```
pub fn set_style(sty: TextStyle){
    let _id = ToPrimitive::to_u8(&sty);
    let mut id = 0;
    if let Some(_idv) = _id { id = _idv; }
    print!("\x1B[00m");
    let mut colorcode = String::from("\x1B[0");
    colorcode.push_str(&format!("{}", id));
    colorcode.push_str("m");
    print!("{}", colorcode);
    print!("\x1B[3{}m", FG_COL.load(Ordering::Relaxed));
    print!("\x1B[4{}m", BG_COL.load(Ordering::Relaxed));
}
/// Print to stdout, it is just  print!("{}", msg);
/// Here to stay consistent
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::print("cool and good");
/// ```
pub fn print<T: std::fmt::Display>(msg: T){
    print!("{}", msg);
}
/// Print to stdout, it is just  println!("{}", msg);
/// Here to stay consistent
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::println("cool and good");
/// ```
pub fn println<T: std::fmt::Display>(msg: T){
    println!("{}", msg);
}
/// Print to stdout with a text colour.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::print_col("orang no!", tbl::UserColour::Yellow);
/// ```
pub fn print_col<T: std::fmt::Display>(msg: T, col: UserColour){
    set_colour(col, FGBG::FG);
    print!("{}", msg);
}
/// Print to stdout with a text colour.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_col("orang no!", tbl::UserColour::Yellow);
/// ```
pub fn println_col<T: std::fmt::Display>(msg: T, col: UserColour){
    set_colour(col, FGBG::FG);
    println!("{}", msg);
}
/// Print to stdout with text and background colours.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::print_cols("No vegetal!", tbl::UserColour::Green, tbl::UserColour::Black);
/// ```
pub fn print_cols<T: std::fmt::Display>(msg: T, fg: UserColour, bg: UserColour){
    set_colours(fg, bg);
    print!("{}", msg);
}
/// Print to stdout with text and background colours.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_cols("No vegetal!", tbl::UserColour::Green, tbl::UserColour::Black);
/// ```
pub fn println_cols<T: std::fmt::Display>(msg: T, fg: UserColour, bg: UserColour){
    set_colours(fg, bg);
    println!("{}", msg);
}
/// Print to stdout with text style.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::print_style("I am bold.", tbl::TextStyle::Bold);
/// ```
pub fn print_style<T: std::fmt::Display>(msg: T, sty: TextStyle){
    set_style(sty);
    print!("{}", msg);
}
/// Print to stdout with text style.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_style("I am bold.", tbl::TextStyle::Bold);
/// ```
pub fn println_style<T: std::fmt::Display>(msg: T, sty: TextStyle){
    set_style(sty);
    println!("{}", msg);
}
/// Print to stdout with text and background colours and style.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::print_cols_style("No vegetal!", tbl::UserColour::Green, tbl::UserColour::Black, tbl::TextStyle::Bold);
/// ```
pub fn print_cols_style<T: std::fmt::Display>(msg: T, fg: UserColour, bg: UserColour, sty: TextStyle){
    set_colours(fg, bg);
    set_style(sty);
    print!("{}", msg);
}
/// Print to stdout with text and background colours and style.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_cols_style("No vegetal!", tbl::UserColour::Green, tbl::UserColour::Black, tbl::TextStyle::Bold);
/// ```
pub fn println_cols_style<T: std::fmt::Display>(msg: T, fg: UserColour, bg: UserColour, sty: TextStyle){
    set_colours(fg, bg);
    set_style(sty);
    println!("{}", msg);
}
/// Returns the character as u8 typed by the user. 
/// It will return immediately after being typed, without the user pressing 'enter'.
///
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// //print user input until spacebar is pressed
/// loop{
///     let x = tbl::getch();
///     if x == 32 { break; }
///     print!("{}", x as char);
/// }
/// ```
pub fn getch() -> u8{
    //https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios;
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();
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
pub fn test_chars(){
    loop {
        println!("{:?}", getch());
    }
}

/// A struct that holds information about a history of typed input's but the user.
pub struct InputHistory{
    history: VecDeque<String>,
    maxlen: usize,
}

impl InputHistory{
    /// Make a new InputHistory with a certain maximum capacity.
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basic_linux as tbl;
    /// let mut his = tbl::InputHistory::new(10);
    /// let x = tbl::input_field_scrollable(&mut his);
    /// ```
    pub fn new(maxlen: usize) -> Self{
        Self{
            history: VecDeque::new(),
            maxlen
        }
    }

    fn trim(&mut self){
        while self.history.len() > self.maxlen{
            self.history.pop_front();
        }
    }
    /// Adds an element to the history.
    /// It will delete items if it's length would grow past the max length.
    /// the oldest items will be removed.
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basics_linux as tbl;
    /// let mut his = tbl::InputHistory::new(2);
    /// his.add(&"0".to_string());
    /// his.add(&"1".to_string());
    /// his.add(&"2".to_string());
    /// //only "1" and "2" will remain, as 0 is removed.
    /// let _ = tbl::input_field_scrollable(&mut his);
    /// ```
    pub fn add(&mut self, string: &String){
        self.history.push_back(string.clone());
        self.trim();
    }
    /// Returns the an Option of String, the element at the given index.
    /// The index wraps and you can query for negative indices as well as indices above the maximum length.
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basics_linux as tbl;
    /// let mut his = tbl::InputHistory::new(3);
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
    pub fn get_index(&self, mut index: i32) -> Option<&String>{
        if self.history.len() != 0 {
            index %= self.history.len() as i32;
        }
        if index < 0{
            index += self.maxlen as i32;
        }
        self.history.get(index as usize)
    }
}

/// Lets the user type text. It returns the string after the user presses 'enter'.
/// It supports moving the cursor with the arrow keys, 
/// going to the begin and end of the line using 'home' and 'end'
/// and deleting characters with backspace and the delete key.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// let user_input = tbl::input_field();
/// ```
pub fn input_field() -> String{
    input_field_scrollable(&mut InputHistory::new(0))
}

/// Lets the user type text. It returns the string after the user presses 'enter'.
/// It supports all functions ```input_field()``` supports.
/// It also supports scrolling through the history of earlier typed strings with
/// the 'up' and 'down' arrow keys.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// let mut history = tbl::InputHistory::new(100);
/// let input0 = tbl::input_field_scrollable(&mut history);
/// println!("You typed: {}", input0);
/// let input1 = tbl::input_field_scrollable(&mut history);
/// println!("You typed: {}", input1);
/// ```

pub fn input_field_scrollable(history: &mut InputHistory) -> String{
    fn charvec_to_string(vec: &[char]) -> String{
        let mut string = String::new();
        for &ch in vec {
            string.push(ch);
        }
        string
    }
    fn typed_char(ch: u8, buff: &mut Vec<char>, gstate: &mut u8, hstate: &mut u8, pos: &mut usize){
        let ch = ch as char;
        buff.insert(*pos, ch);
        if *pos != buff.len() - 1{
            for item in buff.iter().skip(*pos){
                print!("{}", item);
            }
            for _ in  *pos..buff.len()-1{
                print!("{}", 8 as char);
            }
        }else{
            print!("{}", ch);
        }
        *hstate = 0;
        *gstate = 0;
        *pos += 1;
    }
    fn delete_all(buff: &mut Vec<char>) -> usize{
        for _ in 0..buff.len(){
            print!("{}", 8 as char);
        }
        let len = buff.len();
        buff.clear();
        len
    }
    fn feed_into_buffer(buff: &mut Vec<char>, string: &str){
        for ch in string.chars(){
            buff.push(ch);
        }
    }
    fn write_all(buff: &Vec<char>){
        for item in buff.iter(){
            print!("{}", item);
        }
    }
    fn scroll_action(res: &mut Vec<char>, pos: &mut usize, history: &InputHistory, his_index: i32){
        let val = history.get_index(his_index);
        if let Some(valv) = val{
            let old_len = delete_all(res);
            feed_into_buffer(res, valv);
            write_all(&res);
            *pos = res.len();
            let diff = old_len as i32 - res.len() as i32;
            if diff <= 0 { return; }
            for _ in 0..diff{
                print!(" ");
            }
            for _ in 0..diff{
                print!("{}", 8 as char);
            }
        }
    }
    fn delete(res: &mut Vec<char>, pos: &mut usize, gstate: &mut u8){
        if res.is_empty() { return; }
        if *pos >= res.len() - 1 { return; }
        res.remove(*pos);
        for item in res.iter().skip(*pos){
            print!("{}", item);
        }
        print!(" ");
        for _ in *pos..res.len()+1{
            print!("{}", 8 as char);
        }
        *gstate = 0;
    }
    fn end(res: &mut Vec<char>, pos: &mut usize, hoen_state: &mut u8){
        for _ in *pos..res.len() {
            print!("\x1B[1C");
        }
        *pos = res.len();
        *hoen_state = 0;
    }

    let mut res = Vec::new();
    let mut gstate: u8 = 0;
    let mut hoen_state: u8 = 0;
    let mut pos = 0;
    let mut his_index: i32 = 0;

    //set_colour(MsgType::Normal);
    loop {
        let mut x = getch();
        if x == 8 { x = 127; } //shift + backspace
        match x{
            10 => { println!(); break; } //enter
            127 => {  //backspace
                if res.is_empty() { continue; }
                if pos <= 0 { continue; }
                res.remove(pos - 1);
                print!("{}", 8 as char);
                //print!("\x1B[1D"); //also works
                for item in res.iter().skip(pos-1){
                    print!("{}", item);
                }
                print!(" ");
                for _ in pos-1..res.len()+1{
                    print!("{}", 8 as char);
                }
                pos -= 1;
                gstate = 0;
            }
            27 => { //first char in arrow code and home/end code and other char combos
                gstate = 1;
                hoen_state = 1;
            } 
            91 => { //2nd char in arrow code and home/end code and other char combos
                if gstate == 1 { gstate = 2; }
                if hoen_state == 1 { hoen_state = 2; }
                if gstate == 2 || hoen_state == 2 { continue; }
                typed_char(91, &mut res, &mut gstate, &mut hoen_state, &mut pos);
            }
            65 => { //up arrow
                if gstate == 2 {
                    gstate = 0; 
                    his_index += 1;
                    scroll_action(&mut res, &mut pos, &history, his_index);
                }
                else { typed_char(65, &mut res, &mut gstate, &mut hoen_state, &mut pos); }
            }
            66 => { //down arrow
                if gstate == 2 {
                    gstate = 0; 
                    his_index -= 1;
                    scroll_action(&mut res, &mut pos, &history, his_index);
                }
                else { typed_char(66, &mut res, &mut gstate, &mut hoen_state, &mut pos); }
            }
            72 => { //home key
                if hoen_state != 2 { 
                    typed_char(72, &mut res, &mut gstate, &mut hoen_state, &mut pos);
                    continue;
                }
                for _ in 0..pos {
                    print!("{}", 8 as char);
                }
                pos = 0;
                hoen_state = 0;
            }
            51 => {
                if gstate != 2 {
                    typed_char(51, &mut res, &mut gstate, &mut hoen_state, &mut pos);
                }
                else {
                    gstate = 3;
                }
            }
            52 => { //end key 3e char
                if hoen_state == 2 { hoen_state = 3; }
                else { typed_char(52, &mut res, &mut gstate, &mut hoen_state, &mut pos); }
            }
            70 =>{ //end(27-91-70)
                if hoen_state == 2 {
                    end(&mut res, &mut pos, &mut hoen_state);
                }
                else{
                    typed_char(70, &mut res, &mut gstate, &mut hoen_state, &mut pos);
                }
            }
            126 => { //end(27-91-52-126) or delete(27-91-51-126)
                if hoen_state == 3 { //end
                    end(&mut res, &mut pos, &mut hoen_state);
                }
                else if gstate >= 2{ //delete
                    delete(&mut res, &mut pos, &mut gstate);
                }
                else {
                    typed_char(126, &mut res, &mut gstate, &mut hoen_state, &mut pos);
                }
                
            }
            80 => { //delete key with code 27-91-80
                if gstate != 2 { 
                    typed_char(80, &mut res, &mut gstate, &mut hoen_state, &mut pos);
                    continue;
                }
                delete(&mut res, &mut pos, &mut gstate);
            }
            67 => {  //right arrow
                if gstate == 2 {
                    if pos < res.len() { print!("\x1B[1C"); }
                    gstate = 0;
                    pos = min(pos + 1, res.len());
                }
                else { typed_char(67, &mut res, &mut gstate, &mut hoen_state, &mut pos); }
            }
            68 => {  //left arrow
                if gstate == 2 {
                    if pos > 0 { print!("{}", 8 as char); }
                    gstate = 0;
                    pos = max(pos as i32 - 1, 0 as i32) as usize;
                }
                else { typed_char(68, &mut res, &mut gstate, &mut hoen_state, &mut pos); }
            }
            x => { typed_char(x, &mut res, &mut gstate, &mut hoen_state, &mut pos); }
        }
    }
    let string = charvec_to_string(&res);
    history.add(&string);
    string
}

pub fn string_to_bool(string: &str) -> bool{
    match string{
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
}

/// Small helper to parse string to a value
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// let user_input = tbl::prompt("type your age: ");
/// let age: Option<u8> = tbl::string_to_int(&user_input);
/// if age.is_none() { println!("Invalid age!"); }
/// else { println!("Your age: {}", age.unwrap()); }
/// ```
/// Uses ```string.parse::<T>();```
pub fn string_to_value<T: std::str::FromStr>(string: &str) -> Option<T>{
    let res = string.parse::<T>();
    if res.is_err() { return Option::None; }
    res.ok()
}

/// Flushes stdout. 
/// When you do print! or term-basics-linux equivalent, it will not print immediately.
/// For example if you print! and then input_field(), it will print after you typed in the input.
/// flush() will make sure everything is printed first.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// print("type: ");
/// flush().expect("oh no");
/// let x = input_field();
/// ```
/// 
/// This example is the same as ``` let x = tbl::prompt("type: "); ```
pub fn flush() -> io::Result<()>{
    std::io::stdout().flush()
}

/// Prints a message to the user.
/// The user can  type its input next to the message on the same line.
/// It will return the user input after the user pressed enter.
/// It uses term_basics_linux::tbl::input_field and supports the same operation.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// let name = tbl::prompt("type your name: ");
/// tbl::print("Your name: ");
/// tbl::println(name);
/// ```
pub fn prompt(msg : &str) -> String{
    print(msg);
    flush().expect("Error: stdout flush failed.");
    input_field()
}
