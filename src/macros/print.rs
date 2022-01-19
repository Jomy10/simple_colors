
#[macro_export]
/// Prints a color to the screen with a new line
///
/// # Examples
/// ```rust
/// # use simple_colors::{printlnc, red, Color};
/// # fn main() {
/// // Following three calls are equal
/// println!("{}", red!("This is red"));
/// printlnc!(Color::Red, "This is red");
/// printlnc!(red!("This is red"));
/// # }
/// ```
///
/// You can also use this macro to print (custom) styles
macro_rules! printlnc {
    ( $style: expr, $str: tt) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        println!("{}", format!("{}{}\x1b[0m", $style.get_style_code(), $str))
    });
    ( $text: expr ) => ({
        println!("{}", $text)
    })
}

#[macro_export]
/// Prints a color to the screen
///
/// # Examples
/// ```rust
/// # use simple_colors::{printc, red, Color};
/// # fn main() {
/// // Following three calls are equal
/// print!("{}", red!("This is red"));
/// printc!(Color::Red, "This is red");
/// printc!(red!("This is red"));
/// # }
/// ```
macro_rules! printc {
    ( $style: expr, $str: tt) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        print!("{}", format!("{}{}\x1b[0m", $style.get_style_code(), $str))
    });
    ( $text: expr ) => ({
        print!("{}", $text)
    })
}
