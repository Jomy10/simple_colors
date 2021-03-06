//! Simple colors provides macros for styling text with colors, backgrounds and styles like bold,
//! italic and underline.
//!
//! ![Licenses](https://img.shields.io/crates/l/simple_colors)
//!
//! <div>
//!     <img alt="green" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/green.gif" width="400"/>
//!     <img alt="all" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/all.gif" width="400"/>
//!     <img alt="bg" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/bg_blue.gif" width="400"/>
//!     <img alt="bold" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/bold.gif" width="400"/>
//! </div>
//!
//! # Usage
//! ```rust
//! # use simple_colors::{white, red, printlnc};
//! # fn main() {
//! println!("{}", red!("This is red"));
//! printlnc!(red!("This is also red"));
//! printlnc!(format!("{}, {}.", white!("This is white"), red!("this is red")))
//! # }
//! ```
//!
//! <img alt="red_output" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/img/red_ex.png" width="500"/>
//!
//! ```rust
//! # use simple_colors::{color, red, Color};
//! # fn main() {
//! println!("{}", color!(Color::Red, "This is red"));
//! println!("{}", red!("This will be the same color"));
//! pritlnc!(bg_red("This text has a red background"));
//! # }
//! ```
//!
//! ```rust
//! # use simple_colors::{bold, green};
//! # fn main() {
//! println!("{}", bold!(green!("This text is bold and green")));
//! # }
//! ```
//!
//! <img alt="output" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/img/ex1.png" width="500"/>
//!
//! # Define your own styles
//! You can create your own styles like:
//!
//! ```rust
//! # use simple_colors::{color, Style, Color};
//! # fn main() {
//! struct MyCustomStyle;
//! impl simple_colors::custom::Style for MyCustomStyle {
//!     fn get_style_code(&self) -> String {
//!         // This will return a code for bold and light blue text
//!         format!("{}{}",
//!             Style::Bold.get_style_code(),
//!             Color::LightBlue.get_style_code()
//!         )
//!     }
//! }
//!
//!   println!("{}", color!(MyCustomStyle,
//! 		 "This text is light blue and bold, \
//! 	 	but on some terminals it is purple."))
//! # }
//! ```
//!
//! <img alt="output" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/img/custom_ex1.png" width="500"/>
//!
//! ```rust
//! # use simple_colors::{color, Color, Style};
//! # fn main() {
//! enum MyCustomStyles {
//!     Style1,
//!     Style2
//! }
//! impl simple_colors::custom::Style for MyCustomStyles {
//!     fn get_style_code(&self) -> String {
//!         match self {
//!             // Style1 will be bold and light blue
//!             MyCustomStyles::Style1 => "\x1b[1m\x1b[94m".to_string(),
//!             // Style2 will be bold and red
//!             MyCustomStyles::Style2 =>
//!                 format!(
//!                     "{}{}",
//!                     Style::Bold.get_style_code(),
//!                     Color::Red.get_style_code()
//!                 )
//!         }
//!     }
//! }
//!
//!  println!("{}", color!(MyCustomStyles::Style2, "Some text that is both bold and red"))
//! # }
//! ```
//!
//! <img alt="output" src="https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/img/custom_ex2.png" width="500"/>
//!

mod macros;
pub use macros::*;
pub mod custom;
mod enums;
pub use enums::*;
