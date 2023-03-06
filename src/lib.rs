use lightningcss::error::ParserError;

pub mod parse;
/// Get the window bar theme currently used by [GTK](https://www.gtk.org/).

pub fn get() {
    "~/.config/gtk-4.0/gtk.css";
    std::env::var("GTK_THEME").unwrap();
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParserError(String),
}
