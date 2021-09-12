//! # Examples
//! ```
//! use term_basics_linux as tbl;
//! for sty in tbl::TextStyle::iterator(){
//!     for bg in tbl::UC::iterator(){
//!         for fg in tbl::UC::iterator(){
//!             tbl::println_cols_style("cool and good", *fg, *bg, *sty);
//!         }
//!     }
//! }
//! ```
//! ```
//! use term_basics_linux as tbl;
//! let name = tbl::prompt("type your name: ");
//! tbl::print("Your name: ");
//! tbl::println(name);
//! ```
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
static STYLE: AtomicU8 = AtomicU8::new(0);
static USE_NEWLINE_PROMPT: AtomicU8 = AtomicU8::new(0);

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

/// User Colours(UC) available. The user has defined the exact values of
/// these colours for there TTY or emulator.
#[derive(PartialEq,Eq,Clone,Copy,ToPrimitive)]
pub enum UC {
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

impl UC {
    /// Iterate over all colours in the enum
    ///
    /// # Example
    ///
    /// ```
    /// use term_basics_linux as tbl;
    /// for col in tbl::UC::iterator(){
    ///     //use col
    /// }
    /// ```
    pub fn iterator() -> Iter<'static, Self> {
        static ARR: [UC; 9] = [
            UC::Std,
            UC::Black,
            UC::Red,
            UC::Green,
            UC::Yellow,
            UC::Blue,
            UC::Magenta,
            UC::Cyan,
            UC::Grey];
        ARR.iter()
    }
}

impl std::fmt::Display for UC{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", _get_colour(*self, XG::FG, false))
    }
}

/// All styles that do not alter fg or bg colours.
#[derive(PartialEq,Eq,Clone,Copy,ToPrimitive)]
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
#[derive(PartialEq,Eq,Clone,Copy)]
pub enum XG { FG, BG }
/// Sets the colour of the text printed after this call.
/// It will print linux colour escape characters to std out.
/// It will set the state, so you can use ```restore_colour``` to get it back.
///
/// # Examples
///
/// ```
/// use term_basics_linux as tbl;
/// for i in tbl::UC::iterator(){
///     tbl::set_colour(i.clone(), tbl::XG::FG);
///     println!("haha yes");
/// }
/// ```
/// ```
/// use term_basics_linux as tbl;
/// for i in tbl::UC::iterator(){
///     tbl::set_colour(i.clone(), tbl::XG::BG);
///     println!("haha yes");
/// }
/// ```
pub fn set_colour(col: UC, fgbg: XG){
    print!("{}", _get_colour(col, fgbg, true));
}
/// Sets the colour of the text printed after this call.
/// It will print linux colour escape characters to std out.
/// It will not set the state, so you can not use ```restore_colour``` to get this state back.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::use_colour(tbl::UC::Blue, tbl::XG::FG);
/// tbl::println("henlo frens");
/// ```
pub fn use_colour(col: UC, fgbg: XG){
    print!("{}", _get_colour(col, fgbg, false));
}

fn _get_colour(col: UC, fgbg: XG, update_state: bool) -> String{
    let _id = ToPrimitive::to_u8(&col);
    let mut id = 0;
    if let Some(_idv) = _id { id = _idv; }
    let mut colorcode = String::from("\x1B[");
    if fgbg == XG::FG {
        colorcode.push('3');
        if update_state{
            FG_COL.store(id, Ordering::Relaxed);
        }
    }else{
        colorcode.push('4');
        if update_state{
            BG_COL.store(id, Ordering::Relaxed);
        }
    }
    colorcode.push_str(&format!("{}", id));
    colorcode.push('m');
    colorcode
}
/// Restores the colour from the state.
/// ```set_colour``` will set the state and use colour will not.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::set_colour(tbl::UC::Red, tbl::XG::FG);
/// tbl::println("this is red");
/// tbl::use_colour(tbl::UC::Green, tbl::XG::FG);
/// tbl::println("this is green");
/// tbl::restore_colour(tbl::XG::FG);
/// tbl::println("this is red again");
/// ```
pub fn restore_colour(fgbg: XG){
    let mut colorcode = String::from("\x1B[");
    if fgbg == XG::FG{
        colorcode.push('3');
        colorcode.push_str(&format!("{}", FG_COL.load(Ordering::Relaxed)));
    }else{
        colorcode.push('4');
        colorcode.push_str(&format!("{}", BG_COL.load(Ordering::Relaxed)));
    }
    colorcode.push('m');
    print!("{}", colorcode);
}

/// Sets both foreground and background colours.
/// It will print linux colour escape characters to std out.
/// It will set the state.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// for fg in tbl::UC::iterator(){
///     for bg in tbl::UC::iterator(){
///         tbl::set_colours(fg.clone(), bg.clone());
///         println!("haha yes");
///     }
/// }
/// ```
pub fn set_colours(fg: UC, bg: UC){
    set_colour(fg, XG::FG);
    set_colour(bg, XG::BG);
}
/// Sets both foreground and background colours.
/// It will print linux colour escape characters to std out.
/// It will not set the state.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// for fg in tbl::UC::iterator(){
///     for bg in tbl::UC::iterator(){
///         tbl::use_colours(fg.clone(), bg.clone());
///         println!("haha yes");
///     }
/// }
/// ```
pub fn use_colours(fg: UC, bg: UC){
    use_colour(fg, XG::FG);
    use_colour(bg, XG::BG);
}
/// Restores all colours from state.
/// It is used after a call like ```use_colour``` to get back to the old colours.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::set_colours(tbl::UC::Green, tbl::UC::Magenta);
/// tbl::println("cool and good");
/// tbl::use_colours(tbl::UC::Red, tbl::UC::Black);
/// tbl::println("warm and bad");
/// tbl::restore_colours();
/// tbl::println("cool and good again");
/// ```
pub fn restore_colours(){
    restore_colour(XG::FG);
    restore_colour(XG::BG);
}
/// Sets the style of the text printed after this call.
/// It will print linux colour escape characters to std out.
/// It will also set the state.
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
    _set_style(sty, true);
}
/// Sets the style of the text printed after this call.
/// It will print linux colour escape characters to std out.
/// It will not set the state so you can not restore it.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// for i in tbl::TextStyle::iterator(){
///     tbl::use_style(i.clone());
///     println!("haha yes");
/// }
/// ```
pub fn use_style(sty: TextStyle){
    _set_style(sty, false);
}

fn _set_style(sty: TextStyle, update_state: bool){
    let _id = ToPrimitive::to_u8(&sty);
    let mut id = 0;
    if let Some(_idv) = _id { id = _idv; }
    if update_state{
        STYLE.store(id, Ordering::Relaxed);
    }
    print!("\x1B[00m");
    let mut colorcode = String::from("\x1B[0");
    colorcode.push_str(&format!("{}", id));
    colorcode.push('m');
    print!("{}", colorcode);
    print!("\x1B[3{}m", FG_COL.load(Ordering::Relaxed));
    print!("\x1B[4{}m", BG_COL.load(Ordering::Relaxed));
}

/// Restores the style from the state.
/// The state is set after calls like ```set_style```
/// Usually you restore it after a call like ```use_style```.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::set_style(tbl::TextStyle::Bold);
/// tbl::println("this is bold");
/// tbl::use_style(tbl::TextStyle::Crossed);
/// tbl::println("this is crossed");
/// tbl::restore_style();
/// tbl::println("this is bold again");
/// ```
pub fn restore_style(){
    print!("\x1B[00m");
    let mut stylecode = String::from("\x1B[0");
    stylecode.push_str(&format!("{}", STYLE.load(Ordering::Relaxed)));
    stylecode.push('m');
    print!("{}", stylecode);
    print!("\x1B[3{}m", FG_COL.load(Ordering::Relaxed));
    print!("\x1B[4{}m", BG_COL.load(Ordering::Relaxed));
}

/// Resets all the colours.
/// It set the foreground and background colours to the standard colours, whatever they may be.
/// This depends on your terminal emulator and or settings like bashrc or zshrc.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::set_colours(tbl::UC::Magenta, tbl::UC::Yellow);
/// tbl::set_style(tbl::TextStyle::Underlined);
/// tbl::println("i am magenta on yellow as well as underlined");
/// tbl::reset_colours();
/// tbl::println("i am underlined but have standard colours")
/// ```
pub fn reset_colours(){
    FG_COL.store(9, Ordering::Relaxed);
    BG_COL.store(9, Ordering::Relaxed);
    restore_style();
}

/// Resets the style.
/// It sets the style to the default style.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::set_colours(tbl::UC::Cyan, tbl::UC::Red);
/// tbl::set_style(tbl::TextStyle::Blink);
/// tbl::println("im am cyan on red and blinking");
/// tbl::reset_style();
/// tbl::println("i am still cyan on red but im am not blinking");
/// ```
pub fn reset_style(){
    STYLE.store(0, Ordering::Relaxed);
    print!("\x1B[00m");
    restore_colours();
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
/// tbl::print_col("orang no!", tbl::UC::Yellow);
/// ```
pub fn print_col<T: std::fmt::Display>(msg: T, col: UC){
    use_colour(col, XG::FG);
    print!("{}", msg);
    restore_colour(XG::FG);
}
/// Print to stdout with a text colour.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_col("orang no!", tbl::UC::Yellow);
/// ```
pub fn println_col<T: std::fmt::Display>(msg: T, col: UC){
    use_colour(col, XG::FG);
    println!("{}", msg);
    restore_colour(XG::FG);
}
/// Print to stdout with text and background colours.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::print_cols("No vegetal!", tbl::UC::Green, tbl::UC::Black);
/// ```
pub fn print_cols<T: std::fmt::Display>(msg: T, fg: UC, bg: UC){
    use_colours(fg, bg);
    print!("{}", msg);
    restore_colours();
}
/// Print to stdout with text and background colours.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_cols("No vegetal!", tbl::UC::Green, tbl::UC::Black);
/// ```
pub fn println_cols<T: std::fmt::Display>(msg: T, fg: UC, bg: UC){
    use_colours(fg, bg);
    println!("{}", msg);
    restore_colours();
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
    use_style(sty);
    print!("{}", msg);
    restore_style();
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
    use_style(sty);
    println!("{}", msg);
    restore_style();
}
/// Print to stdout with text and background colours and style.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::print_cols_style("No vegetal!", tbl::UC::Green, tbl::UC::Black, tbl::TextStyle::Bold);
/// ```
pub fn print_cols_style<T: std::fmt::Display>(msg: T, fg: UC, bg: UC, sty: TextStyle){
    use_style(sty);
    use_colours(fg, bg);
    print!("{}", msg);
    restore_colours();
    restore_style();
}
/// Print to stdout with text and background colours and style.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::println_cols_style("No vegetal!", tbl::UC::Green, tbl::UC::Black, tbl::TextStyle::Bold);
/// ```
pub fn println_cols_style<T: std::fmt::Display>(msg: T, fg: UC, bg: UC, sty: TextStyle){
    use_style(sty);
    use_colours(fg, bg);
    println!("{}", msg);
    restore_colours();
    restore_style();
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
    let reader = io::stdin();
    let mut buffer = [0;1];
    stdout.lock().flush().unwrap();
    reader.lock().read_exact(&mut buffer).unwrap();
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
    /// use term_basics_linux as tbl;
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
    pub fn add(&mut self, string: &str){
        self.history.push_back(string.to_string());
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
        if !self.history.is_empty(){
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
    input_field_raw(&mut InputHistory::new(0), PromptChar::Copy)
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
    input_field_raw(history, PromptChar::Copy)
}

/// Lets the user type text. It returns the string after the user presses 'enter'.
/// It supports all functions ```input_field()``` supports.
/// You can specify your own ```InputHistory``` and ```PromptChar```.
///
/// # Example
///
/// ```use term_basics_linux as tbl;
/// let password = tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Substitude('*')); //Hide the users password as it is typed in!
/// tbl::println_style(password, tbl::TextStyle::Bold); // THAN PRINT IT OUT
/// ```
pub fn input_field_custom(his: &mut InputHistory, pc: PromptChar) -> String{
    input_field_raw(his, pc)
}

/// Call this before ```input_field``` or it's variations if you want to NOT print a newline(```\n```) after the user presses enter.
/// This will work for the next time you call any version of ```input_field```.
/// To cancel it you can call ```use_newline_on_prompt```.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::discard_newline_on_prompt_nexttime();
/// let _ = tbl::prompt("enter your name: ");
/// tbl::println(" // your name");
/// ```
pub fn discard_newline_on_prompt_nexttime(){
    USE_NEWLINE_PROMPT.store(1, Ordering::Relaxed);
}

/// Call this to let any variation of ```input_field``` print a newline after the user presses enter.
/// This is not needed, they will print a newline by default.
/// This is used to cancel ```discard_newline_on_prompt_nexttime```.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::discard_newline_on_prompt_nexttime();//use somewhere
/// tbl::use_newline_on_prompt();//cancel somewhere else in code
/// let _ = tbl::prompt("enter your name: ");
/// tbl::println(" // your name");
/// ```
pub fn use_newline_on_prompt(){
    USE_NEWLINE_PROMPT.store(0, Ordering::Relaxed);
}
/// What kind of character the prompt will print.
/// ```Copy``` will just print what the user types in.
/// ```Substitude(char)``` will print that character.
/// ```None``` will not print anything at all.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// tbl::println(tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Copy));
/// tbl::println(tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::Substitude('#')));
/// tbl::println(tbl::input_field_custom(&mut tbl::InputHistory::new(0), tbl::PromptChar::None));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PromptChar{
    Copy,
    Substitude(char),
    None,
}

fn input_field_raw(history: &mut InputHistory, pc: PromptChar) -> String{
    fn charvec_to_string(vec: &[char]) -> String{
        let mut string = String::new();
        for &ch in vec {
            string.push(ch);
        }
        string
    }
    fn typed_char(ch: u8, buff: &mut Vec<char>, gstate: &mut u8, hstate: &mut u8, pos: &mut usize, pc: PromptChar){
        let ch = ch as char;
        buff.insert(*pos, ch);
        if pc != PromptChar::None{
            if *pos != buff.len() - 1{
                for item in buff.iter().skip(*pos){
                    put_char(*item, pc);
                }
                go_back(*pos,buff.len()-1,pc);
            }else{
                put_char(ch, pc);
            }
        }
        *hstate = 0;
        *gstate = 0;
        *pos += 1;
    }
    fn delete_all(buff: &mut Vec<char>, pc: PromptChar) -> usize{
        if pc == PromptChar::None {
            buff.clear();
            return 0;
        }
        go_back(0, buff.len(), pc);
        let len = buff.len();
        buff.clear();
        len
    }
    fn feed_into_buffer(buff: &mut Vec<char>, string: &str){
        for ch in string.chars(){
            buff.push(ch);
        }
    }
    fn write_all(buff: &[char], pc: PromptChar){
        for item in buff.iter(){
            put_char(*item, pc);
        }
    }
    fn scroll_action(res: &mut Vec<char>, pos: &mut usize, history: &InputHistory, his_index: i32, pc: PromptChar){
        let val = history.get_index(his_index);
        if let Some(valv) = val{
            feed_into_buffer(res, valv);
            *pos = res.len();
            if pc == PromptChar::None { return; }
            let old_len = delete_all(res, pc);
            write_all(res, pc);
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
    fn delete(res: &mut Vec<char>, pos: &mut usize, gstate: &mut u8, pc: PromptChar){
        if res.is_empty() { return; }
        if *pos >= res.len() - 1 { return; }
        res.remove(*pos);
        for item in res.iter().skip(*pos){
            put_char(*item, pc);
        }
        if pc != PromptChar::None { print!(" "); }
        go_back(*pos,res.len()+1, pc);
        *gstate = 0;
    }
    fn end(res: &mut Vec<char>, pos: &mut usize, hoen_state: &mut u8, pc: PromptChar){
        if pc != PromptChar::None {
            for _ in *pos..res.len() {
                print!("\x1B[1C");
            }
        }
        *hoen_state = 0;
        *pos = res.len();
    }
    fn put_char(ch: char, pc: PromptChar){
        match pc{
            PromptChar::Copy => print!("{}", ch),
            PromptChar::Substitude(sch) => print!("{}", sch),
            PromptChar::None => {},
        };
    }
    fn go_back(start: usize, end: usize, pc: PromptChar){
        if pc == PromptChar::None { return; }
        for _ in  start..end{
            print!("{}", 8 as char);
        }
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
            10 => {  //enter
                if USE_NEWLINE_PROMPT.load(Ordering::Relaxed) == 0 {
                    println!();
                }else{
                    USE_NEWLINE_PROMPT.store(0, Ordering::Relaxed);
                }
                break;
            }
            127 => {  //backspace
                if res.is_empty() { continue; }
                if pos == 0 { continue; }
                res.remove(pos - 1);
                if pc != PromptChar::None {
                    print!("{}", 8 as char);
                    //print!("\x1B[1D"); //also works
                    for item in res.iter().skip(pos-1){
                        put_char(*item, pc);
                    }
                    print!(" ");
                    go_back(pos-1,res.len()+1,pc);
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
                typed_char(91, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
            }
            65 => { //up arrow
                if gstate == 2 {
                    gstate = 0;
                    his_index += 1;
                    scroll_action(&mut res, &mut pos, history, his_index, pc);
                }
                else { typed_char(65, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            66 => { //down arrow
                if gstate == 2 {
                    gstate = 0;
                    his_index -= 1;
                    scroll_action(&mut res, &mut pos, history, his_index, pc);
                }
                else { typed_char(66, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            72 => { //home key
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
            52 => { //end key 3e char
                if hoen_state == 2 { hoen_state = 3; }
                else { typed_char(52, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            70 =>{ //end(27-91-70)
                if hoen_state == 2 {
                    end(&mut res, &mut pos, &mut hoen_state, pc);
                }
                else{
                    typed_char(70, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                }
            }
            126 => { //end(27-91-52-126) or delete(27-91-51-126)
                if hoen_state == 3 { //end
                    end(&mut res, &mut pos, &mut hoen_state, pc);
                }
                else if gstate >= 2{ //delete
                    delete(&mut res, &mut pos, &mut gstate, pc);
                }
                else {
                    typed_char(126, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                }
            }
            80 => { //delete key with code 27-91-80
                if gstate != 2 {
                    typed_char(80, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc);
                    continue;
                }
                delete(&mut res, &mut pos, &mut gstate, pc);
            }
            67 => {  //right arrow
                if gstate == 2 {
                    let old_pos = pos;
                    pos = min(pos + 1, res.len());
                    gstate = 0;
                    if pc == PromptChar::None { continue; }
                    // if pos < res.len() { print!("\x1B[1C"); }
                    if pos > old_pos { print!("\x1B[1C"); }
                }
                else { typed_char(67, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            68 => {  //left arrow
                if gstate == 2 {
                    let old_pos = pos;
                    pos = max(pos as i32 - 1, 0_i32) as usize;
                    gstate = 0;
                    if pc == PromptChar::None { continue; }
                    // if pos > 0 { print!("{}", 8 as char); }
                    if pos < old_pos { print!("{}", 8 as char); }
                }
                else { typed_char(68, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
            }
            x => { typed_char(x, &mut res, &mut gstate, &mut hoen_state, &mut pos, pc); }
        }
    }
    let string = charvec_to_string(&res);
    history.add(&string);
    string
}

pub fn string_to_bool(string: &str) -> bool{
    matches!(string, "y" | "ye" | "yes" | "ok" | "+" | "t" | "tr" | "tru" | "true")
}

/// Small helper to parse string to a value
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// let user_input = tbl::prompt("type your age: ");
/// let age: Option<u8> = tbl::string_to_value(&user_input);
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
/// use std::io::Write; //flush stdout
/// use term_basics_linux as tbl;
/// print!("type: ");
/// tbl::flush().expect("oh no");
/// let x = tbl::input_field();
/// ```
///
/// This example is the same as ``` let x = tbl::prompt("type: "); ```
pub fn flush() -> io::Result<()>{
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
/// let name = tbl::prompt("type your name: ");
/// tbl::print("Your name: ");
/// tbl::println(name);
/// ```
pub fn prompt(msg : &str) -> String{
    print(msg);
    flush().expect("term_basics_linux: Error: stdout flush failed.");
    input_field()
}

/// Prints a message to the user.
/// The user can type its input next to the message on the same line.
/// It will return the user input after the user pressed enter.
/// It uses term_basics_linux::input_field_scrollable and supports the same operations.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// let mut his = tbl::InputHistory::new(10);
/// his.add(&"previously typed in name".to_string());
/// let name = tbl::prompt_scrollable("type your name: ", &mut his);
/// tbl::print("Your name: ");
/// tbl::println(name);
/// ```
pub fn prompt_scrollable(msg: &str, his: &mut InputHistory) -> String{
    print(msg);
    flush().expect("term_basics_linux: Error: stdout flush failed.");
    input_field_scrollable(his)
}

/// Prints a message to the user.
/// The user can type its input next to the message on the same line.
/// It will return the user input after the user pressed enter.
/// It uses term_basics_linux::input_field_scrollable and supports the same operations.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// let password = tbl::prompt_masked("Enter password: ", '*'); // Hide the users password as it is typed in!
/// tbl::println_style(password, tbl::TextStyle::Blink); // show it to the world with some extra spice.
/// ```
pub fn prompt_masked(msg: &str, ch: char) -> String{
    print(msg);
    flush().expect("term_basics_linux: Error: stdout flush failed.");
    input_field_custom(&mut InputHistory::new(0), PromptChar::Substitude(ch))
}

/// Prints a message to the user.
/// The user can type its input next to the message on the same line.
/// It will return the user input after the user pressed enter.
/// It uses term_basics_linux::input_field_custom and supports the same operations.
/// So you can provide your own InputHistory and PromptChar.
///
/// # Example
///
/// ```
/// use term_basics_linux as tbl;
/// let mut his = tbl::InputHistory::new(2);
/// his.add(&"hidden option 0".to_string());
/// his.add(&"hidden option 1".to_string());//provide options but the user can't see them.
/// let x = tbl::prompt_custom("enter input:", &mut his, tbl::PromptChar::None);
/// tbl::println(x);
/// ```
pub fn prompt_custom(msg: &str, his: &mut InputHistory, pc: PromptChar) -> String{
    print(msg);
    flush().expect("term_basics_linux: Error: stdout flush failed.");
    input_field_custom(his, pc)
}
