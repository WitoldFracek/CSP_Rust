#![allow(dead_code)]

pub mod colors {

    pub const END: &str = "\x1b[0m";
    pub const NONE: &str = "";

    #[derive(Copy, Clone)]
    pub struct RGBColor {
        pub r: usize,
        pub g: usize,
        pub b: usize,
    }

    impl RGBColor {
        pub fn new(r: usize, g: usize, b: usize) -> Self {
            RGBColor {
                r, g, b
            }
        }
    }

    pub fn colored_text(text: &str, fg: &str, bg: &str, has_end: bool) -> String {
        if has_end {
            format!("{}{}{}{}", fg, bg, text, END)
        } else {
            format!("{}{}{}{}", fg, bg, text, "")
        }
    }


    pub fn bg_color(r: usize, g: usize, b: usize) -> String {
        return format!("\x1b[48;2;{};{};{}m", r, g, b)
    }

    pub mod style {
        pub const BOLD: &str = "\x1b[1m";
        pub const ITALIC: &str = "\x1b[3m";
        pub const URL: &str = "\x1b[4m";
        pub const BLINK: &str = "\x1b[5m";
        pub const BLINK2: &str = "\x1b[6m";
        pub const SELECTED: &str = "\x1b[7m";
    }

    pub mod fg {
        pub const BLACK: &str = "\x1b[30m";
        pub const RED: &str = "\x1b[31m";
        pub const GREEN: &str = "\x1b[32m";
        pub const YELLOW: &str = "\x1b3[33m";
        pub const BLUE: &str = "\x1b[34m";
        pub const VIOLET: &str = "\x1b[35m";
        pub const BEIGE: &str = "\x1b[36m";
        pub const WHITE: &str = "\x1b[37m";
        pub const ORANGE: &str = "\x1b[38;2;255;128;0m";

        pub fn color(r: usize, g: usize, b: usize) -> String {
            return format!("\x1b[38;2;{};{};{}m", r, g, b)
        }
    }

    pub mod bg {
        pub const BLACK: &str = "\x1b[40m";
        pub const RED: &str = "\x1b[41m";
        pub const GREEN: &str = "\x1b[42m";
        pub const YELLOW: &str = "\x1b[43m";
        pub const BLUE: &str = "\x1b[44m";
        pub const VIOLET: &str = "\x1b[45m";
        pub const BEIGE: &str = "\x1b[46m";
        pub const WHITE: &str = "\x1b[47m";
        pub const ORANGE: &str = "\x1b[48;2;255;153;51m";

        pub fn color(r: usize, g: usize, b: usize) -> String {
            return format!("\x1b[48;2;{};{};{}m", r, g, b)
        }
    }
}

pub mod sudoku {
    use crate::presentation::colors::{self};
    use crate::presentation::colors::colored_text;
    use crate::problem::grid::Grid;

    pub fn sudoku_repr(sudoku: &Grid<u8>) -> String {
        let n = (sudoku.size as f64).sqrt() as usize;
        let mut ret = String::new();
        let mut bool_start = true;
        for (i, row) in sudoku.domains.iter().enumerate() {
            let mut is_white = bool_start;
            for (j, domain) in row.iter().enumerate() {
                let val = match domain.value() {
                    None => String::from("   "),
                    Some(val) => format!(" {} ", val),
                };
                if is_white {
                    if domain.is_fixed() {
                        ret = format!("{ret}{}", colored_text(&val, colors::fg::RED, colors::bg::WHITE, true))
                    } else {
                        ret = format!("{ret}{}", colored_text(&val, colors::fg::BLACK, colors::bg::WHITE, true))
                    }
                } else {
                    if domain.is_fixed() {
                        ret = format!("{ret}{}", colored_text(&val, colors::fg::RED, colors::bg::BLACK, true))
                    } else {
                        ret = format!("{ret}{}", colored_text(&val, colors::fg::WHITE, colors::bg::BLACK, true))
                    }
                }
                if (j + 1) % n == 0 {
                    is_white = !is_white;
                }
            }
            ret.push('\n');
            if (i + 1) % n == 0 {
                bool_start = !bool_start;
            }
        }
        ret
    }
}