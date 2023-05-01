use std::slice::Iter;

mod conversion;

pub enum MorseChar {
  Dit,
  Dah
};

pub struct Morsec<'input> {
    data: Option<&'input str>, // TODO: convert this into an appropriate iterator owned by the struct
    toggle: Option<fn>, // TODO: make this have the appropriate traits
};


impl <'input> Morsec<'input> {
  fn new() -> Self {
    todo!();
  }

  fn input(&mut self, in: &str) {
    todo!();
  }
}

pub const fn convert_char(input: char) -> Iter<'static, MorseChar> {

} 

struct MorseIter {};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
