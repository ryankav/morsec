enum MorseChar {
  Dit,
  Dah
};

pub struct Morsec<'input> {

};


impl <'input> Morsec<'input> {
  fn new() -> Self {
    todo!();
  }

  fn input(&mut self, in: &str) {
    todo!();
  }
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
