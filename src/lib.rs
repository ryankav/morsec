use std::slice::Iter;

mod conversion;
use conversion::*;

#[derive(Clone, Copy, Debug)]
pub enum MorseChar {
    Dit,
    Dah,
}

pub struct Morsec<'input> {
    _data: Option<&'input str>, // TODO: convert this into an appropriate iterator owned by the struct
    _toggle: Option<fn()>,      // TODO: make this have the appropriate traits
}

impl<'input> Morsec<'input> {
    fn new() -> Self {
        todo!();
    }

    fn input(&mut self, input: &str) {
        todo!();
    }
}

pub const fn convert_char(input: char) -> Iter<'static, MorseChar> {
    match input {
        _ => unimplemented!(),
    }
}

struct MorseIter {}
