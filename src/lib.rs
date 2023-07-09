use std::slice::Iter;
use std::str::SplitWhitespace;
use std::thread::sleep;
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
    message: SplitWhitespace<'input>,

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
            message: message.split_whitespace(),
            toggle,
            dit_duration: Duration::from_millis(500),
        }
    }

    pub fn set_space_duration(&mut self, duration: Duration) {
        self.dit_duration = duration;
    }

    /// Transmit will send the message that was given to the struct and in the
    /// process consume the struct
    pub fn transmit(mut self) {
        for word in self.message {
            for letter in word.chars() {
                for symbol in convert_char(letter) {
                    (self.toggle)();
                    match symbol {
                        MorseChar::Dit => sleep(self.dit_duration),
                        MorseChar::Dah => sleep(3 * self.dit_duration),
                    };
                    (self.toggle)();
                    // Sleep between the encoded characters
                    sleep(self.dit_duration);
                }
                // sleep between letters in the words cumulative needs to be 3x
                // the dit duration
                sleep(2 * self.dit_duration);
            }
            // Sleep so the time between words is cumulative 7x dit duration
            sleep(4 * self.dit_duration);
        }
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
