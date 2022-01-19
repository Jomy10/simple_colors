#[macro_export]
/// Adds a background to text. See also the different bg_{color} methods to be able to display
/// text with a background and text color (e.g. [`bg_blue!`])
///
/// # Example
/// ```rust
/// # use simple_colors::{green, bg, printlnc};
/// # fn main() {
/// printlnc!(bg!(green!("This text has a green background")))
/// # }
/// ```
macro_rules! bg {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[7m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[7m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// Adds a background to text
///
/// # Example
/// ```rust
/// # use simple_colors::{green, bg_black, bold, printlnc};
/// # fn main() {
/// printlnc!(bg_black!(green!(bold!("This text has a black background and bold, green text"))))
/// # }
/// ```
macro_rules! bg_black {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[40m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[40m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_red {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[41m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[41m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_green {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[42m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[42m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_yellow {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[43m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[43m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![bg_blue](https://github.com/jomy10/simple_colors/assets/gif/bg_blue.gif)
macro_rules! bg_blue {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[44m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[44m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_magenta {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[45m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[45m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_cyan {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[46m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[46m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_gray {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[47m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[47m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_dark_gray {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[100m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[100m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_red {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[101m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[101m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_green {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[102m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[102m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_yellow {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[103m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[103m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_blue {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[104m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[104m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_magenta {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[105m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[105m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_light_cyan {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[106m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[106m{}\x1b[0m", $other)
    })
}

#[macro_export]
macro_rules! bg_white {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[107m{}\x1b[0m", $str)
    });
    // Allows to call other style macros inside of this macro
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[107m{}\x1b[0m", $other)
    })
}