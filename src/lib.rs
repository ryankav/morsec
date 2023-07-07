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
        'a' | 'A' => A.iter(),
        'b' | 'B' => B.iter(),
        'c' | 'C' => C.iter(),
        'd' | 'D' => D.iter(),
        'e' | 'E' => E.iter(),
        'f' | 'F' => F.iter(),
        'g' | 'G' => G.iter(),
        'h' | 'H' => H.iter(),
        'i' | 'I' => I.iter(),
        'j' | 'J' => J.iter(),
        'k' | 'K' => K.iter(),
        'l' | 'L' => L.iter(),
        'm' | 'M' => M.iter(),
        'n' | 'N' => N.iter(),
        'o' | 'O' => O.iter(),
        'p' | 'P' => P.iter(),
        'q' | 'Q' => Q.iter(),
        'r' | 'R' => R.iter(),
        's' | 'S' => S.iter(),
        't' | 'T' => T.iter(),
        'u' | 'U' => U.iter(),
        'v' | 'V' => V.iter(),
        'w' | 'W' => W.iter(),
        'x' | 'X' => X.iter(),
        'y' | 'Y' => Y.iter(),
        'z' | 'Z' => Z.iter(),
        _ => unimplemented!(),
    }
}

struct MorseIter {}
