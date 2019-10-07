[![Crate](https://img.shields.io/crates/v/term-basics-linux.svg)](https://crates.io/crates/term-basics-linux)
[![API](https://img.shields.io/crates/v/term-basics-linux.svg?color=blue&label=docs)](https://docs.rs/term-basics-linux/0.2.1/term_basics_linux/tbl/index.html)
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
You can aslo mail to codybloemhard@gmail.com
## links
This readme is used on multiple sites so some links might be redundant.
* [https://github.com/ocdy1001/term-basics-linux](https://github.com/ocdy1001/term-basics-linux)
* [https://crates.io/crates/term-basics-linux](https://crates.io/crates/term-basics-linux)
* [https://docs.rs/term-basics-linux/](https://docs.rs/term-basics-linux/)
