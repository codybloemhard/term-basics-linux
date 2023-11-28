# term-basics-linux

Rust crate library that provides basic terminal input functionality for Linux.
The main feature is the single line input field.
It supports editing the text with backspace and delete,
as well as navigating using home, end and the arrow keys.

```rust
print!("your input: ");
let input = tbl::input_field_simple(true);
```

It supports a list of predefined inputs for the user to scroll between (up/down arrow):

```rust
let mut his = tbl::InputList::new(2);
his.add("one");
his.add("two");
println!("{}", tbl::input_field_scrollable(&mut his, true));
```

You can also hide/substitute the typed characters.
This is handy for password input:

```rust
// like doas/sudo
let pass = tbl::input_field(&mut tbl::InputList::new(0), tbl::PrintChar::None, true);
// like websites
let pass = tbl::input_field(&mut tbl::InputList::new(0), tbl::PrintChar::Substitute('*'), true);
```

Another feature is ```getch()``` which returns the characters from stdin without
the user having to press enter.
It can be useful and is not available by default in rust.
All other input fields are using this function.

## design

This crate is very minimalistic by design, It does just a few things.
It is for Linux only. This keeps it simple and lightweight.
For a 

## keycodes

Sometimes different terminal emulators use different codes for certain keys like delete or end.
For example, backspace is 127 on both suckless simple terminal (ST)
and the build in terminal emulator in vscode.
But delete is 27-91-80 on ST and 27-91-51-126 on vscode.
End is 27-91-52-126 on ST and 27-91-70 on vscode.
The ```test_chars``` function can help look up what key code it is on your platform.

## links

* [https://gitlab.com/codybloemhard/term-basics-linux](https://gitlab.com/codybloemhard/term-basics-linux)
* [https://crates.io/crates/term-basics-linux](https://crates.io/crates/term-basics-linux)
* [https://docs.rs/term-basics-linux/](https://docs.rs/term-basics-linux/)

## License

```
Copyright (C) 2023 Cody Bloemhard

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
