use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use crate::math::digits::DigitsError::{NoConversionCharToNumRule, NoConversionNumToCharRule};

pub enum DigitsError{
    DigitExceedLimit(u8,u8),
    NoConversionCharToNumRule(char),
    NoConversionNumToCharRule(u8),
}
impl Debug for DigitsError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DigitsError::DigitExceedLimit(d,max) => {
                write!(f, "digits exceeded limit ({d} out of {max})")
            }
            NoConversionCharToNumRule(c) => {
                write!(f, "no conversion rule for char '{c}' to num")
            }
            NoConversionNumToCharRule(n) => {
                write!(f, "no conversion rule for num [{n}] to char")
            }
        }
    }
}
pub const fn arabic_num_to_char(digit:u8) ->Option<char>{
    if(digit<=9){
        Some((b'0'+digit) as char)
    }else if(digit<=35) {
        Some((b'a'+digit-10) as char)
    }else if(digit<=61) {
        Some((b'A'+digit-36) as char)
    }else{
        None
    }
}
pub const fn char_to_arabic_num(digit:char) ->Option<u8>{
    if(digit>='0'&&digit<='9'){
        Some((digit as u8)-b'0')
    }else if(digit>='a'&&digit<='z') {
        Some((digit as u8)-b'a'+10)
    }else if(digit>='A'&&digit<='Z') {
        Some((digit as u8)-b'A'+36)
    }else{
        None
    }
}
pub struct Digits{
    digits: Vec<u8>,
    max_digit: u8,
}
impl Digits{
    pub fn new(max_digit: u8) -> Digits{
        Digits{
            digits:Vec::new(),
            max_digit,
        }
    }
    #[inline]
    pub fn len(&self) -> usize{
        self.digits.len()
    }
    #[inline]
    pub fn append(&mut self, digit: u8)->Result<(), DigitsError>{
        if(digit>=self.max_digit){
            Err(DigitsError::DigitExceedLimit(digit,self.max_digit))
        }else{
            Ok(self.digits.push(digit))
        }
    }
    pub fn from_string(digits_string: &str, max_digit: u8, convert_rule: fn(char) ->Option<u8>) -> Result<Digits,DigitsError>{
        let mut digit=Digits::new(max_digit);
        let mut digits_chars = digits_string.chars().collect::<Vec<char>>();
        digits_chars.reverse();
        for digit_char in digits_chars{
            digit.append(match convert_rule(digit_char) {
                Some(n) => n,
                None => return Err(DigitsError::NoConversionCharToNumRule(digit_char)),
            })?;
        }
        Ok(digit)
    }
    pub fn to_string(&self,convert_rule: fn(u8)->Option<char>) -> Result<String,DigitsError>{
        let mut result = String::new();
        let digits_nums =self.digits.iter().rev();
        for digit in digits_nums {
            result.push(match convert_rule(*digit) {
                Some(c) => c,
                None => return Err(DigitsError::NoConversionNumToCharRule(*digit)),
            } );
        }
        Ok(result)
    }
    pub fn to_string_complex(&self,convert_rule: fn(u8)->Option<String>) -> Option<String>{
        let mut result = String::new();
        let digits_nums =self.digits.iter().rev();
        for digit in digits_nums {
            match convert_rule(*digit) {
                Some(c) => result.push_str(c.as_str()),
                None => return None,
            }
        }
        Some(result)
    }

    pub fn cast_to_string(&self) -> String{
        fn convert_rule(value: u8) -> Option<String>{
            match arabic_num_to_char(value) {
                Some(v)=>Some(v.to_string()),
                None=> Some(format!("[{}]",value.to_string()))
            }
        }
        match self.to_string_complex(convert_rule){
            Some(v)=>{v},
            None=>{panic!("Impossible")}
        }
    }
    pub fn from_u64(mut value:u64, max_digit:u8) -> Digits{
        let mut digits=Digits::new(max_digit);
        while(value>0){
            digits.append((value % max_digit as u64) as u8);
            value/=max_digit as u64;
        }
        digits
    }
    #[inline]
    pub fn get_u8_array(&self)->&Vec<u8>{
        &self.digits
    }
}
impl Index<usize> for Digits{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digits[(index)]
    }
}
impl IndexMut<usize> for Digits{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.digits[(index)]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_134(){
        let num="134";
        let digits = Digits::from_string(num,10,char_to_arabic_num).unwrap();
        assert_eq!(digits.to_string(arabic_num_to_char).unwrap(),num)
    }
    #[test]
    fn test_2978(){
        let num=2978;
        let digits = Digits::from_u64(num,10);
        assert_eq!(digits.to_string(arabic_num_to_char).unwrap(),num.to_string())
    }
    #[test]
    fn test_114514(){
        let num=114514;
        let digits=Digits::from_u64(num, 10);
        assert_eq!(digits.to_string(arabic_num_to_char).unwrap(),"114514");
    }
}
