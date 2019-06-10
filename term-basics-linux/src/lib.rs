#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
/// # Examples
/// ```
/// use term_basics_linux::tbl;
/// for sty in tbl::TextStyle::iterator(){
///     for bg in tbl::UserColour::iterator(){
///         for fg in tbl::UserColour::iterator(){
///             tbl::set_style(sty.clone());
///             tbl::set_colour(bg.clone(), tbl::FGBG::BG);
///             tbl::set_colour(fg.clone(), tbl::FGBG::FG);
///             println!("haha yes");
///         }
///     }
/// }
/// ```
/// ```
/// let name = tbl::prompt("type your name: ");
/// tbl::print("Your name: ");
/// tbl::println(name);
/// ```
pub mod tbl{
    use std::io;
    use std::io::Write; //flush stdout
    use std::io::Read;
    use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
    use std::cmp::{ max, min };
    use num_derive::ToPrimitive;
    use num_traits::ToPrimitive;
    use std::slice::Iter;
    /// Colours available. The user has defined the exact values of 
    /// these colours for there TTY or emulator.
    #[derive(PartialEq,Eq,Clone,ToPrimitive)]
    pub enum UserColour {
        Std     = 99,
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
    /// use term_basics_linux::tbl;
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
            ARR.into_iter()
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
    /// use term_basics_linux::tbl;
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
            ARR.into_iter()
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
    /// use term_basics_linux::tbl;
    /// for i in tbl::UserColour::iterator(){
    ///     tbl::set_colour(i.clone(), tbl::FGBG::FG);
    ///     println!("haha yes");
    /// }
    /// ```
    /// ```
    /// use term_basics_linux::tbl;
    /// for i in tbl::UserColour::iterator(){
    ///     tbl::set_colour(i.clone(), tbl::FGBG::BG);
    ///     println!("haha yes");
    /// }
    /// ```
    pub fn set_colour(col: UserColour, fgbg: FGBG){
        if col == UserColour::Std{
            print!("\x1B[00m");
        }
        let _id = ToPrimitive::to_u8(&col);
        let mut id = 0;
        if _id.is_some() { id = _id.unwrap(); }
        let mut colorcode = String::from("\x1B[");
        if fgbg == FGBG::FG {
            colorcode.push('3');
        }else{
            colorcode.push('4');
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
    /// for i in tbl::TextStyle::iterator(){
    ///     tbl::set_style(i.clone());
    ///     println!("haha yes");
    /// }
    /// ```
    pub fn set_style(sty: TextStyle){
        let _id = ToPrimitive::to_u8(&sty);
        let mut id = 0;
        if _id.is_some() { id = _id.unwrap(); }
        let mut colorcode = String::from("\x1B[0");
        colorcode.push_str(&format!("{}", id));
        colorcode.push_str("m");
        print!("{}", colorcode);
    }
    /// Print to stdout, it is just  print!("{}", msg);
    /// Here to stay consistent
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// use term_basics_linux::tbl;
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
    /// Prints the result of getch as u8, infinite loop. Can be used for testing.
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basics_linux::tbl;
    /// let user_input = tbl::test_chars();
    /// ```
    pub fn test_chars(){
        loop {
            print!("{:?}\n", getch());
        }
    }
    /// Lets the user type text. It returns the string after the user presses 'enter'.
    /// It supports moving the cursor with the arrow keys, 
    /// going to the begin and end of the line using 'home' and 'end'
    /// and deleting characters with backspace.
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basics_linux::tbl;
    /// let user_input = tbl::input_field();
    /// ```
    pub fn input_field() -> String{
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

        //set_colour(MsgType::Normal);
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
    /// Prints a message to the user.
    /// The user can  type its input next to the message on the same line.
    /// It will return the user input after the user pressed enter.
    /// It uses term_basics_linux::tbl::input_field and supports the same operation.
    /// 
    /// # Example
    /// 
    /// ```
    /// use term_basics_linux::tbl;
    /// let name = tbl::prompt("type your name: ");
    /// tbl::print("Your name: ");
    /// tbl::println(name);
    /// ```
    pub fn prompt(msg : &str) -> String{
        //print_type(msg, MsgType::Prompt);
        std::io::stdout().flush().expect("Error: stdout flush failed.");
        return input_field();
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
}