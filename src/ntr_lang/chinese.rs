use crate::dprintln;
use crate::math::digits::Digits;
use crate::ntr_lang::lang::{LanguageParser, NumToLangParser, NumberConvertError};

pub struct Chinese {
}
impl LanguageParser for Chinese{
    fn name() -> &'static str {
        return "Chinese";
    }
}
pub struct NumberToChineseParser {
    ///prefer 零 over 〇
    prefer_ling:bool,
    ///prefer 一十 over 十
    prefer_one_ten:bool,
    ///control whether to use traditional
    traditional:bool,
}
///units of 10^4 in simplified chinese
const MEGA_UNITS: [&str; 12] = [
    "", "万", "亿", "兆", "京", "垓", "秭", "穰", "沟", "涧", "正", "载"
];
///units of 10^4 in traditional chinese
const MEGA_UNITS_TRAD: [&str; 12] = [
    "", "萬", "億", "兆", "京", "垓", "秭", "穰", "溝", "澗", "正", "載"
];
///digits in both Chinese versions
const DIGITS: [char; 9] = [
    '一','二','三','四','五','六','七','八','九',
];

impl NumberToChineseParser {
    pub fn default()-> NumberToChineseParser {
        NumberToChineseParser {
            prefer_ling:true,
            prefer_one_ten:false,
            traditional:false,
        }
    }
    ///create a new Chinese parser with customized options
    pub fn new(prefer_ling:bool, prefer_one_ten:bool, traditional:bool)-> NumberToChineseParser {
        NumberToChineseParser {
            prefer_ling,
            prefer_one_ten,
            traditional
        }
    }
    pub fn megaunit(&self, place: usize) -> Result<&'static str, NumberConvertError> {
        if (!self.traditional) {
            if place < MEGA_UNITS.len() {
                return Ok(MEGA_UNITS[place])
            }
        } else {
            if place < MEGA_UNITS_TRAD.len() {
                return Ok(MEGA_UNITS_TRAD[place])
            }
        }
        Err(NumberConvertError::NumberOutOfRange)
    }
    pub fn digit_to_char(&self,digit: u8) -> char {
        if (digit > 0 && digit <= 9) {
            DIGITS[digit as usize-1]
        } else {
            self.zero()
        }
    }
    pub fn zero(&self) -> char {
        if(self.prefer_ling){
            '零'
        }else{
            '〇'
        }
    }
}
impl NumToLangParser for NumberToChineseParser {
    fn number_to_text(&self, num: Digits) -> Result<String, NumberConvertError> {
        let last_digit= num.len()-1;
        let mut text = String::new();
        let mut zeros:usize=0;
        for (place,digit) in num.get_u8_array().iter().enumerate() {
            //counting zeros
            if(*digit==0){
                //if it is the first place zero but
                //there is no zero in the smaller section, we still need to add zero
                if(place%4==0&&place!=0){
                    if(zeros==0){
                        text.push(self.zero());
                    }
                }
                zeros+=1;
            }else{
                zeros=0;
            }
            //if is first digit in the section or in the number, we should do some work
            if(place%4==3||place==last_digit){
                let section_start=place-place%4;
                //check if the unit is not used, like 1_0000_0000 do not display 万
                if(zeros>=4){
                    //if the whole section is 0, skip it
                    continue;
                }else{
                    //enter the normal process
                    text.push_str(self.megaunit(place/4)?);
                    let mut section_zeros:usize=0;
                    for (section_place,section_digit) in num.get_u8_array()
                        [section_start..=place].iter().enumerate(){
                        if(*section_digit==0){
                            //check is there any hanging zeros, like 1001
                            //if it is the rightmost place of 0, we should consider adding 零
                            if(section_zeros==0){
                                //do not add 零 when it is the rightmost digit in the section
                                //but add it elsewhere
                                if(section_place != 0) {
                                    text.push(self.zero());
                                }
                            }
                            //record one more zero
                            section_zeros+=1;
                        }else{
                            //reset section zeros
                            section_zeros=0;
                            //enter the normal number process
                            //add the approximate unit in section
                            match section_place{
                                0=>(),
                                1=>{
                                    text.push('十');
                                    dprintln!("place: {place}; section_place:{section_place}; last digit: {last_digit}; section digit: {section_digit}; prefer yishi: {}",self.prefer_one_ten);
                                    if((*section_digit==1)&&(!self.prefer_one_ten)&&(section_start+section_place==last_digit)){
                                        continue;
                                    }
                                },
                                2=>{
                                    text.push('百');
                                },
                                3=>{
                                    text.push('千');
                                },
                                _=>{
                                    debug_assert!(false, "should not have section place of: {:?}",section_place );
                                }
                            }
                            text.push(self.digit_to_char(*section_digit));
                        }
                    }
                }
            }
        }
        Ok(text.chars().rev().collect::<String>())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn make(num: u64) -> String {
        NumberToChineseParser::default()
            .number_to_text(Digits::from_u64(num, 10))
            .unwrap()
    }

    #[test]
    fn test_single_digits() {
        // 基础单位测试
        let expected = [
            "", "一", "二", "三", "四", "五", "六", "七", "八", "九"
        ];
        for (i, &ch) in expected.iter().enumerate().skip(1) {
            assert_eq!(make(i as u64), ch, "digit {} failed", i);
        }
    }

    #[test]
    fn test_teens_and_tens() {
        assert_eq!(make(10), "十");
        assert_eq!(make(11), "十一");
        assert_eq!(make(20), "二十");
        assert_eq!(make(21), "二十一");
        assert_eq!(make(99), "九十九");
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(make(100), "一百");
        assert_eq!(make(101), "一百零一");
        assert_eq!(make(110), "一百一十");
        assert_eq!(make(111), "一百一十一");
        assert_eq!(make(999), "九百九十九");
    }
    #[test]
    fn test_thousands() {
        assert_eq!(make(1000), "一千");
        assert_eq!(make(1001), "一千零一");
        assert_eq!(make(1010), "一千零一十");
        assert_eq!(make(1100), "一千一百");
        assert_eq!(make(9999), "九千九百九十九");
    }

    #[test]
    fn test_wan_yi_sections() {
        // 测试跨节单位
        assert_eq!(make(10_000), "一万");
        assert_eq!(make(10_001), "一万零一");
        assert_eq!(make(10_010), "一万零一十");
        assert_eq!(make(10_100), "一万零一百");
        assert_eq!(make(100_000), "十万");
        assert_eq!(make(999_999), "九十九万九千九百九十九");
    }

    #[test]
    fn test_yi_and_zhao() {
        // 综合性测试（comprehensive「全面的」）
        assert_eq!(make(100_000_000), "一亿");
        assert_eq!(make(1_0000_0001), "一亿零一");
        assert_eq!(make(1_0100_0000), "一亿零一百万");
        assert_eq!(make(2_0000_0000), "二亿");
        assert_eq!(make(10_0000_0000), "十亿");
        assert_eq!(make(1_0000_0000_0000), "一兆");
    }

    #[test]
    fn test_large_magnitudes() {
        assert_eq!(
            make(1_1451_4191_9810),
            "一兆一千四百五十一亿四千一百九十一万九千八百一十"
        );
        assert_eq!(
            make(1234_5678_9012_3456_7890),
            "一千二百三十四京五千六百七十八兆九千零一十二亿三千四百五十六万七千八百九十"
        );
    }

    #[test]
    fn test_zero_behavior() {
        // 连续零、省略零、边界零
        assert_eq!(make(10005), "一万零五");
        assert_eq!(make(10050), "一万零五十");
        assert_eq!(make(1005000), "一百万零五千");
        assert_eq!(make(10000000), "一千万");
        assert_eq!(make(1000000000), "十亿");
        assert_eq!(make(1000000000000), "一兆");
    }

    #[test]
    fn test_traditional_mode() {
        // 测试繁体字（如萬、億）
        let trad = NumberToChineseParser {
            prefer_ling: false,
            prefer_one_ten: false,
            traditional: true,
        };
        assert_eq!(
            trad.number_to_text(Digits::from_u64(10_0000_0000, 10)).unwrap(),
            "十億"
        );
        assert_eq!(
            trad.number_to_text(Digits::from_u64(1_0000_0000_0000, 10)).unwrap(),
            "一兆"
        );
    }

    #[test]
    fn test_edge_and_error_cases() {
        let c = NumberToChineseParser {
            prefer_ling: false,
            prefer_one_ten: false,
            traditional: false,
        };
        // 极大单位应返回 Err
        assert!(c.megaunit(999).is_err());
    }
}
