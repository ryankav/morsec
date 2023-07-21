use core::slice::Iter;
use core::str::SplitWhitespace;
use core::time::Duration;

mod conversion;
use conversion::*;

#[derive(Clone, Copy, Debug)]
pub(crate) enum MorseChar {
    Dit,
    Dah,
}

pub struct Morsec<'input, T: MorsecTransmitter> {
    /// The message that is to be transmitted via Morse code, it is required
    /// that the message be one of the 26 basic Latin characters or a space
    message: SplitWhitespace<'input>,

    /// User defined struct that implements the MorsecTransmitter trait
    transmitter: T,
}

pub trait MorsecTransmitter {
    /// The DIT_DURATION controls the time length that a dit will last for.
    /// It is the fundamental time unit in Morse code, as all other times are
    /// defined of this single value. A dah for example being 3 times longer
    /// than a dit, the space between dits/dahs - which are part of the same
    /// letter - lasts the same length of time as a dit and the space between
    /// words is 7 times that of a dit.
    ///
    /// This value is optional as it has been defaulted to be 500ms. It is not
    /// needed for a user to use this variable in their code as the Morsec
    /// library will handle using this variable in the appropriate way
    const DIT_DURATION: Duration = Duration::from_millis(500);

    /// Morse code relies on the time delays to signal messages. As such the
    /// user is required to provide a function which can cause execution to
    /// sleep for the variable duration.
    fn sleep(&mut self, duration: Duration);

    /// As Morse code transmits messages through toggling on and off some
    /// medium, the toggle function here provides a way to allow the Morsec
    /// struct to cause the desired side effect
    fn toggle(&mut self);
}

impl<'input, T: MorsecTransmitter> Morsec<'input, T> {
    /// Creates a new Morsec struct from the input message and the toggle
    /// function. The initial dit_duration is defaulted to be 0.5s
    pub fn new(message: &'input str, transmitter: T) -> Self {
        Self {
            message: message.split_whitespace(),
            transmitter,
        }
    }

    /// Transmit will send the message that was given to the struct and in the
    /// process consume the struct
    pub fn transmit(mut self) {
        for word in self.message {
            for letter in word.chars() {
                for symbol in convert_char(letter) {
                    self.transmitter.toggle();
                    match symbol {
                        MorseChar::Dit => self.transmitter.sleep(T::DIT_DURATION),
                        MorseChar::Dah => self.transmitter.sleep(3 * T::DIT_DURATION),
                    };
                    self.transmitter.toggle();
                    // Sleep between the encoded characters
                    self.transmitter.sleep(T::DIT_DURATION);
                }
                // sleep between letters in the words cumulative needs to be 3x
                // the dit duration
                self.transmitter.sleep(2 * T::DIT_DURATION);
            }
            // Sleep so the time between words is cumulative 7x dit duration
            self.transmitter.sleep(4 * T::DIT_DURATION);
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
