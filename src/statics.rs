#![allow(non_snake_case)]
pub use term::color;

pub static mut ERROR_LINE_COLOR: color::Color = color::RED; // for print error with color(Red)
pub fn ERROR_LINE_COLOR_get() -> u16 {
    unsafe { ERROR_LINE_COLOR }
}
pub fn ERROR_LINE_COLOR_set(num: color::Color) {
    unsafe { ERROR_LINE_COLOR = num }
}

pub static mut OPT_HELP_SORT_KEY: &'static str = "___app_internal_0";
pub fn OPT_HELP_SORT_KEY_get() -> &'static str {
    unsafe { OPT_HELP_SORT_KEY }
}
pub fn OPT_HELP_SORT_KEY_set(s: &'static str) {
    unsafe { OPT_HELP_SORT_KEY = s }
}

pub static mut OPT_VERSION_SORT_KEY: &'static str = "___app_internal_1";
pub fn OPT_VERSION_SORT_KEY_get() -> &'static str {
    unsafe { OPT_VERSION_SORT_KEY }
}
pub fn OPT_VERSION_SORT_KEY_set(s: &'static str) {
    unsafe { OPT_VERSION_SORT_KEY = s }
}

pub static mut OPTIONAL: &'static str = "(optional)";
pub fn OPTIONAL_get() -> &'static str {
    unsafe { OPTIONAL }
}
pub fn OPTIONAL_set(s: &'static str) {
    unsafe { OPTIONAL = s }
}

