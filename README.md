[![Crate](https://img.shields.io/crates/v/term-basics-linux.svg)](https://crates.io/crates/term-basics-linux)
[![API](https://img.shields.io/crates/v/term-basics-linux.svg?color=blue&label=docs)](https://docs.rs/term-basics-linux/)
# term-basics-linux
Rust crate libary that provides simple and basic terminal functionality for linux. (Work In Progress!)
## usage
As the name is quite long, you can do something like this:
```use term_basics_linux as tbl;```
After that you can use the crate:
```let name = tbl::prompt("type your name: ");```
It is recommended to use the ```tbl::``` notation or similar, just like you would not use
```using namespace std;``` in c++.
### examples
The main feature of this crate is the input field, supporting editing your input text with function keys like backspace, delete, home, end, arrows.
```tbl::println(tbl::input_field());```
```
let name = tbl::prompt("type your name: ");
tbl::print("Your name: ");
tbl::println(name);
```
It also supports simple user defined colours and text styles:
```tbl::print_cols_style("very nice", tbl::UserColour::Yellow, tbl::UserColour::Cyan, tbl::TextStyle::Bold);```
Another feature is ```getch()``` which returns the characters from stdin with out the user having to press enter. It can be very useful and is not available by default in rust.
## design
This crate is very simple by design, as it does simple things the code should not be complicated.
It is made for linux, as it is a crate for terminal applications.
With MacOS being a unix based OS and windows having a linux kernel build in, it should be kind of portable-ish.
The crate only supports user defined colours by design.
First of all it keeps this crate simple.
Secondly i think you should only use user defined colours.
If you use them you application will match with the enviroment and other applications.
The user chose there colours for a reason and nobody likes inconsistent colours across applications.
With all the fuss lately around GTK and distro's breaking themes for applications,
this problem is easy to avoid with terminal applications by using the user's colours.
## contribution
You can always create issues and pull requests on github.
You can also mail to codybloemhard@gmail.com
### keycodes
Sometimes different terminal emulators use different codes for certain keys like delete or end.
For example, backspace is 127 on both suckless simple terminal (ST) and the build in terminal emulator in vscode.
But delete is 27-91-80 on ST and 27-91-51-126 on vscode. End is 27-91-52-126 on ST and 27-91-70 on vscode.
If you encounter a non-supported key code, please open an issue on github with the function key(delete,end,etc) and what platform(terminal emulator) you run.
The ```test_chars``` function can help look up what key code it is on your platform. It is helpfull to to supply that information.
Ofcourse you can also make a pull request.
### testing
term-basics-linux is tested on the following terminals (emulators):
| platform                                  | Tested |
| ----------------------------------------- | ------ |
| ST (suckless simple terminal, zsh)        | Always |
| vscode (build in terminal emulator, zsh)  | Always |
| tty (arch linux, zsh)                     | Todo   |
## links
This readme is used on multiple sites so some links might be redundant.
* [https://github.com/ocdy1001/term-basics-linux](https://github.com/ocdy1001/term-basics-linux)
* [https://crates.io/crates/term-basics-linux](https://crates.io/crates/term-basics-linux)
* [https://docs.rs/term-basics-linux/](https://docs.rs/term-basics-linux/)
