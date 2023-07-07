use std::slice::Iter;
use std::str::Chars;
use std::time::Duration;

mod conversion;
use conversion::*;

#[derive(Clone, Copy, Debug)]
pub(crate) enum MorseChar {
    Dit,
    Dah,
}

pub struct Morsec<'input, F: FnMut()> {
    /// The message that is to be transmitted via Morse code, it is required
    /// that the message be one of the 26 basic Latin characters or a space
    message: Chars<'input>,

    /// As Morse code transmits messages through toggling on and off some
    /// medium, the toggle function here provides a way to allow the Morsec
    /// struct to cause the desired side effect
    toggle: F,

    /// Morsec's dit_duration member controls the time length that a dit lasts
    /// for. This is the fundamental time unit in Morse code, as all other
    /// durations are defined of it a dah for example being 3 times as long as
    /// a dit, a space between characters being transmitted is the same as a
    /// dit and the space between words is 7 times that of a dit.
    dit_duration: Duration,
}

impl<'input, F: FnMut()> Morsec<'input, F> {
    /// Creates a new Morsec struct from the input message and the toggle
    /// function. The initial dit_duration is defaulted to be 0.5s
    pub fn new(message: &'input str, toggle: F) -> Self {
        Self {
            message: message.chars(),
            toggle,
            dit_duration: Duration::from_millis(500),
        }
    }

    pub fn set_space_duration(&mut self, duration: Duration) {
        self.dit_duration = duration;
    }

    /// Transmit will send the message that was given to the struct and in the
    /// process consume the struct
    pub fn transmit(self) {
        todo!("implement the actual iteration and sending of the message");
    }
}

fn convert_char(input: char) -> Iter<'static, MorseChar> {
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
