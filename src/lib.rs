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

pub fn convert_char(input: char) -> Iter<'static, MorseChar> {
    match input {
        'A' => A.iter(),
        'B' => B.iter(),
        'C' => C.iter(),
        'D' => D.iter(),
        'E' => E.iter(),
        'F' => F.iter(),
        'G' => G.iter(),
        'H' => H.iter(),
        'I' => I.iter(),
        'J' => J.iter(),
        'K' => K.iter(),
        'L' => L.iter(),
        'M' => M.iter(),
        'N' => N.iter(),
        'O' => O.iter(),
        'P' => P.iter(),
        'Q' => Q.iter(),
        'R' => R.iter(),
        'S' => S.iter(),
        'T' => T.iter(),
        'U' => U.iter(),
        'V' => V.iter(),
        'W' => W.iter(),
        'X' => X.iter(),
        'Y' => Y.iter(),
        'Z' => Z.iter(),
        _ => unimplemented!(),
    }
}

struct MorseIter {}
