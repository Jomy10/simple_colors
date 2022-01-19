#[macro_export]
/// ![black](https://github.com/jomy10/simple_colors/assets/img/black.png)
/// ![black_gif](https://github.com/jomy10/simple_colors/assets/gif/black.gif)
macro_rules! black {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[30m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[30m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![red](https://github.com/jomy10/simple_colors/assets/img/red.png)
/// ![red_gif](https://github.com/jomy10/simple_colors/assets/gif/red.gif)
///
/// Usage:
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
/// ![red_output](https://github.com/jomy10/simple_colors/assets/img/red_ex.png)
macro_rules! red {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[31m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[31m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![green](https://github.com/jomy10/simple_colors/assets/img/green.png)
/// ![green_gif](https://github.com/jomy10/simple_colors/assets/gif/green.gif)
macro_rules! green {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[32m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[32m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![yellow](https://github.com/jomy10/simple_colors/assets/img/yellow.png)
macro_rules! yellow {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[33m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[33m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![blue](https://github.com/jomy10/simple_colors/assets/img/blue.png)
macro_rules! blue {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[34m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[34m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![magenta](https://github.com/jomy10/simple_colors/assets/img/magenta.png)
macro_rules! magenta {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[35m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[35m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![cyan](https://github.com/jomy10/simple_colors/assets/img/cyan.png)
macro_rules! cyan {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[36m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[36m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![grey](https://github.com/jomy10/simple_colors/assets/img/grey.png)
macro_rules! gray {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $other)
    })
}
#[macro_export]
/// ![grey](https://github.com/jomy10/simple_colors/assets/img/grey.png)
macro_rules! grey {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $other)
    })
}
#[macro_export]
/// [`gray!`]
///
/// ![grey](https://github.com/jomy10/simple_colors/assets/img/grey.png)
macro_rules! light_gray {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// [`grey!`]
///
/// ![grey](https://github.com/jomy10/simple_colors/assets/img/grey.png)
macro_rules! light_grey {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[37m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![dark_grey](https://github.com/jomy10/simple_colors/assets/img/dark_grey.png)
macro_rules! dark_gray {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[90m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[90m{}\x1b[0m", $other)
    })
}
#[macro_export]
/// ![dark_grey](https://github.com/jomy10/simple_colors/assets/img/dark_grey.png)
macro_rules! dark_grey {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[90m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[90m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![light_red](https://github.com/jomy10/simple_colors/assets/img/light_red.png)
macro_rules! light_red {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[91m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[92m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![light_green](https://github.com/jomy10/simple_colors/assets/img/light_green.png)
macro_rules! light_green {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[92m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[92m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![light_yellow](https://github.com/jomy10/simple_colors/assets/img/light_yellow.png)
macro_rules! light_yellow {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[93m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[93m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![light_blue](https://github.com/jomy10/simple_colors/assets/img/light_blue.png)
macro_rules! light_blue {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[94m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[94m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![light_magenta](https://github.com/jomy10/simple_colors/assets/img/light_magenta.png)
macro_rules! light_magenta {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[95m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[95m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![light_cyan](https://github.com/jomy10/simple_colors/assets/img/light_cyan.png)
macro_rules! light_cyan {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[96m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[96m{}\x1b[0m", $other)
    })
}

#[macro_export]
/// ![white](https://github.com/jomy10/simple_colors/assets/img/white.png)
macro_rules! white {
    ( $str: tt ) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[97m{}\x1b[0m", $str)
    });
    ( $other: expr) => ({
        use simple_colors::custom::Style as be_jonaseveraert_colors_custom_style;
        &format!("\x1b[97m{}\x1b[0m", $other)
    })
}