/// Formats a string with a color.
///
/// You can also use all the designated color macros (e.g. [`red!`])
///
/// ![red](https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/red.gif)
///
/// # Examples
/// ```rust
/// # use simple_colors::{color, red, Color};
/// # fn main() {
/// println!("{}", color!(Color::Red, "This is text"));
/// println!("{}", red!("This will be the same color"));
/// # }
/// ```
///
/// ```rust
/// # use simple_colors::{color, green, Color};
/// # fn main() {
/// let string = green!("This text is green");
/// println!("{string}");
/// println!("{}", color!(Color::Green, "This text is also green"))
/// # }
/// ```
///
/// ```rust
/// # use simple_colors::{white, red, printlnc};
/// # fn main() {
/// println!("{}", red!("This is red"));
/// printlnc!(red!("This is also red"));
/// printlnc!(format!("{}, {}.", white!("This is white"), red!("this is red")))
/// # }
/// ```
///
/// **Output**
///
/// ![red_output](https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/img/red_ex)
#[macro_export]
macro_rules! color {
        ( $style: expr, $str: tt) => ({
            use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
            &format!("{}{}\x1b[0m", $style.get_style_code(), $str)
        });
        ( $style: expr, $other: expr) => ({
            use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
            &format!("{}{}\x1b[0m", $style.get_style_code(), $other)
        })
    }

/// Formats a string to a color
///
/// # Examples
/// ```rust
/// # use simple_colors::{style, red, Color};
/// # fn main() {
/// println!("{}", style!(Color::Red, "This is red"));
/// println!("{}", red!("This will be the same color"));
/// # }
/// ```
///
/// ```rust
/// # use simple_colors::{style, green, Color};
/// # fn main() {
/// let string = green!("This text is green");
/// println!("{string}");
/// println!("{}", style!(Color::Green, "This text is also green"))
/// # }
/// ```
#[macro_export]
macro_rules! style {
    ( $style: expr, $str: tt) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("{}{}\x1b[0m", $style.get_style_code(), $str)
    })
}