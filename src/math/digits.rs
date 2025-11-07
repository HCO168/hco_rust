use std::error::Error;
use std::num::IntErrorKind::InvalidDigit;
use std::ops::{Index, IndexMut};
use num_traits::FloatErrorKind::Invalid;
use rand::seq::WeightError::InvalidInput;
use crate::containers::map::Map;

pub struct Digits{
    digits: Vec<u8>,
    max_digits: u8,
}
impl Digits{
    fn new(max_digits: u8) -> Digits{
        Digits{
            digits:Vec::new(),
            max_digits,
        }
    }
    fn len(&self) -> usize{
        self.digits.len()
    }
    fn append(&mut self, digit: u8){
        self.digits.push(digit);
    }
    fn from_string(digits_string: String, max_digits: u8, convert_rule: impl Map<char,u8>) -> Result<Digits,impl Error>{
        let mut digit=Digits::new(max_digits);
        let digits_chars = digits_string.chars().collect::<Vec<char>>();
        for digit_char in digits_chars{
            digit.append(convert_rule.get(&digit_char).unwrap_or(return Err(InvalidInput)).clone());
        }
        Ok(digit)
    }
}
impl Index<usize> for Digits{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.digits[(index) as usize];
    }
}
impl IndexMut<usize> for Digits{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        return &mut self.digits[(index) as usize];
    }
}


