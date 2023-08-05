use morsec::{Morsec, MorsecTransmitter};
use std::time::Duration;

#[derive(Default)]
struct MorseTest {
    morse_chars: Vec<u64>,
    sleep_time: Vec<u64>,
}

impl<'a> MorsecTransmitter for &'a mut MorseTest {
    const DIT_DURATION: Duration = Duration::from_secs(1);

    fn sleep(&mut self, duration: Duration) {
        let last_option = if self.morse_chars.len() > self.sleep_time.len() {
            self.morse_chars.last_mut()
        } else {
            self.sleep_time.last_mut()
        };

        if let Some(last) = last_option {
            *last += duration.as_secs();
        }
    }

    fn toggle(&mut self) {
        if self.morse_chars.len() == self.sleep_time.len() {
            self.morse_chars.push(0);
        } else {
            self.sleep_time.push(0);
        }
    }
}

#[test]
fn test_sos() {
    let mut transmitter = MorseTest::default();
    let tester = Morsec::new("sos", &mut transmitter);
    tester.transmit();
    assert_eq!(transmitter.morse_chars, [1, 1, 1, 3, 3, 3, 1, 1, 1]);
    assert_eq!(transmitter.sleep_time, [1, 1, 3, 1, 1, 3, 1, 1, 7]);
}

#[test]
fn test_sentence() {
    let mut transmitter = MorseTest::default();
    let tester = Morsec::new("hello world", &mut transmitter);
    tester.transmit();
    assert_eq!(
        transmitter.morse_chars,
        [
            1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 3, 3, 3, 1, 3, 3, 3, 3, 3, 1, 3, 1, 1, 3, 1, 1,
            3, 1, 1
        ]
    );
    assert_eq!(
        transmitter.sleep_time,
        [
            1, 1, 1, 3, 3, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 7, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 1, 3,
            1, 1, 7
        ]
    );
}
