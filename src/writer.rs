use std::io;
pub trait LineWriter {
    type Error;
    fn write_ln<S: AsRef<str>>(&mut self, s: S) -> Result<(), Self::Error>;
    fn inc(&mut self) {}
    fn dec(&mut self) {}
}

fn spaces(n: i32) -> String {
    let mut res = String::new();
    for _ in 0..n {
        res.push(' ');
    }
    res
}

impl from(

pub struct PrettyIOWriter<W> {
    d: i32,
    w: W,
}

pub struct PrettyFmtWriter<W> {
    d: i32,
    w: W,
}

impl<W: io::Write> LineWriter for PrettyIOWriter<W> {
    type Error = io::Error;
    fn write_ln<S: AsRef<str>>(&mut self, s: S) -> Result<(), io::Error> {
        write!(self.w, "\n{}{}", spaces(self.d), s.as_ref())
    }
    fn inc(&mut self) {
        self.d += 1;
    }
    fn dec(&mut self) {
        self.d -= 1;
    }
}

impl<W: std::fmt::Write> LineWriter for PrettyFmtWriter<W> {
    type Error = std::fmt::Error;
    fn write_ln<S: AsRef<str>>(&mut self, s: S) -> Result<(), std::fmt::Error> {
        write!(self.w, "\n{}{}", spaces(self.d), s.as_ref())
    }
    fn inc(&mut self) {
        self.d += 1;
    }
    fn dec(&mut self) {
        self.d -= 1;
    }
}
