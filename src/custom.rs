/// You can implement your own styles:
///
/// ```rust
/// # use simple_colors::{color, Style, Color};
/// # fn main() {
/// struct MyCustomStyle;
/// impl simple_colors::custom::Style for MyCustomStyle {
///     fn get_style_code(&self) -> String {
///         // This will return a code for bold and light blue text
///         // "\x1b[1m\x1b[94m".to_string()
///         // Which is equivalent to:
///         format!("{}{}",
///             Style::Bold.get_style_code(),
///             Color::LightBlue.get_style_code()
///         )
///     }
/// }
///
///  println!("{}", color!(MyCustomStyle, "My text"))
/// # }
/// ```
///
/// ```rust
/// # use simple_colors::{color, Color, Style};
/// # fn main() {
/// enum MyCustomStyles {
///     Style1,
///     Style2
/// };
/// impl simple_colors::custom::Style for MyCustomStyles {
///     fn get_style_code(&self) -> String {
///         match self {
///             MyCustomStyles::Style1 => "\x1b[1m\x1b[94m".to_string(),
///             // Style2 will be bold and red
///             MyCustomStyles::Style2 =>
///                 format!(
///                     "{}{}",
///                     Style::Bold.get_style_code(),
///                     Color::Red.get_style_code()
///                 )
///         }
///     }
/// }
///
///  println!("{}", color!(MyCustomStyles::Style2, "My text"))
/// # }
/// ```
pub trait Style {
    fn get_style_code(&self) -> String;
}