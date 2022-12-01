#[allow(dead_code)]
#[cfg(windows)]
pub const LINE_ENDING: &'static str = "\r\n";
#[cfg(windows)]
pub const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &'static str = "\n";
#[cfg(not(windows))]
pub const EMPTY_LINE: &'static str = "\n\n";