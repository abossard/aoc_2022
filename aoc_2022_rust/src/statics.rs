#[allow(dead_code)]
#[cfg(windows)]
pub const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
pub const EMPTY_LINE: &'static str = "\n\n";
