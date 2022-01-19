#[macro_export]
/// Makes text bold
///
/// ![bold](https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/bold.gif)
///
/// # Example
/// ```rust
/// # use simple_colors::{bold, red};
/// # fn main() {
///     println!("{}", bold!("This text is bold"));
///     println!("{}", bold!(red!("This text is bold and red")));
/// # }
/// ```
macro_rules! bold {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[1m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[1m{}\x1b[0m", $other)
    })
}

// TODO: example picture
#[macro_export]
/// Darkens text. You can combine this with a color to get a dark variant of this color
///
/// # Example
/// ```rust
/// # use simple_colors::{dark, green, printlnc};
/// # fn main() {
/// printlnc!(dark!(green!("This text is dark green")))
/// # }
/// ```
macro_rules! dark {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[2m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[2m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![italic](https://raw.githubusercontent.com/jomy10/simple_colors/master/assets/gif/italic.gif)
macro_rules! italic {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[3m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[3m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! underline {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[4m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[4m{}\x1b[0m", $other)
    })
}