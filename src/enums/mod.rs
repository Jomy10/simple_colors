pub use self::color::*;
pub use self::style::*;
pub use self::bg::*;

mod color {
    use crate::custom;
    
    pub enum Color {
        Default,
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        LightGray,
        DarkGray,
        LightRed,
        LightGreen,
        LightYellow,
        LightBlue,
        LightMagenta,
        LightCyan,
        White,
    }
    
    impl custom::Style for Color {
        fn get_style_code(&self) -> String {
            match self {
                Color::Default => { "\x1b[0m".to_string() }
                Color::Black => { "\x1b[30m".to_string() }
                Color::Red => { "\x1b[31m".to_string() }
                Color::Green => { "\x1b[32m".to_string() }
                Color::Yellow => { "\x1b[33m".to_string() }
                Color::Blue => { "\x1b[34m".to_string() }
                Color::Magenta => { "\x1b[35m".to_string() }
                Color::Cyan => { "\x1b[36m".to_string() }
                Color::LightGray => { "\x1b[37m".to_string() }
                Color::DarkGray => { "\x1b[90m".to_string() }
                Color::LightRed => { "\x1b[91m".to_string() }
                Color::LightGreen => { "\x1b[92m".to_string() }
                Color::LightYellow => { "\x1b[93m".to_string() }
                Color::LightBlue => { "\x1b[94m".to_string() }
                Color::LightMagenta => { "\x1b[95m".to_string() }
                Color::LightCyan => { "\x1b[96m".to_string() }
                Color::White => { "\x1b[97m".to_string() }
            }
        }
    }
}

mod style {
    use crate::custom;
    
    pub enum Style {
        Bold,
        Dark,
        Italic,
        Underlined
    }
    
    impl custom::Style for Style {
        fn get_style_code(&self) -> String {
            match self {
                Self::Bold => "\x1b[1m".to_string(),
                Self::Dark => "\x1b[2m".to_string(),
                Self::Italic => "\x1b[3m".to_string(),
                Self::Underlined => "\x1b[4m".to_string(),
            }
        }
    }
}

mod bg {
    use crate::custom;
    
    pub enum Background {
        Any,
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        LightGray,
        DarkGray,
        LightRed,
        LightGreen,
        LightYellow,
        LightBlue,
        LightMagenta,
        LightCyan,
        White,
    }
    
    impl custom::Style for Background {
        fn get_style_code(&self) -> String {
            match self {
                Self::Any => "\x1b[7m".to_string(),
                Self::Black => "\x1b[40m".to_string(),
                Self::Red => "\x1b[41m".to_string(),
                Self::Green => { "\x1b[42m".to_string() }
                Self::Yellow => { "\x1b[43m".to_string() }
                Self::Blue => { "\x1b[44m".to_string() }
                Self::Magenta => { "\x1b[45m".to_string() }
                Self::Cyan => { "\x1b[46m".to_string() }
                Self::LightGray => { "\x1b[47m".to_string() }
                Self::DarkGray => { "\x1b[100m".to_string() }
                Self::LightRed => { "\x1b[101m".to_string() }
                Self::LightGreen => { "\x1b[102m".to_string() }
                Self::LightYellow => { "\x1b[103m".to_string() }
                Self::LightBlue => { "\x1b[104m".to_string() }
                Self::LightMagenta => { "\x1b[105m".to_string() }
                Self::LightCyan => { "\x1b[106m".to_string() }
                Self::White => { "\x1b[107m".to_string() }
            }
        }
    }
}