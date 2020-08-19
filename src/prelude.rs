
use crate::ToRandPwd;

pub use heapless;
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use typenum::{ U3, U52, };
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };
pub use std::{
    convert::From,
    ops::{ Add, SubAssign, AddAssign, },
    fmt::{ Display, Formatter, Result, },
};

pub type StrVec = heapless::Vec<String, U52>;
pub type CharVec = heapless::Vec<StrVec, U3>;


lazy_static! {
    /// Cached the characters set
    pub static ref DATA: CharVec = _DATA();
}


/// Characters set
/// return letters, symbols, numbers in `CharVec`
#[inline]
pub(crate) fn _DATA() -> CharVec {

    let mut letters = StrVec::new();
    let mut symbols = StrVec::new();
    let mut numbers = StrVec::new();

    let mut charset = CharVec::new();

    let _ = (33..127)
            .into_iter()
            .map(|x| {
                let ch = x as u8 as char;
                if ch.is_ascii_alphabetic()  { letters.push(ch.to_string()).unwrap(); }
                if ch.is_ascii_punctuation() { symbols.push(ch.to_string()).unwrap(); }
                if ch.is_ascii_digit()       { numbers.push(ch.to_string()).unwrap(); }
            })
            .collect::<()>();

    charset.push(letters).unwrap();
    charset.push(symbols).unwrap();
    charset.push(numbers).unwrap();

    charset

}


/// Count the number of a string
#[inline]
pub(crate) fn _CNT<T: AsRef<str>>(content: T) -> (usize, usize, usize) {

    let mut l = 0;
    let mut s = 0;
    let mut n = 0;

    content.as_ref().chars().for_each(
        |x| {
            if x.is_ascii() {
                if x.is_ascii_alphabetic()  { l += 1; }
                if x.is_ascii_punctuation() { s += 1; }
                if x.is_ascii_digit()       { n += 1; }
            } else {
                panic!("Has non-ASCII character(s)!, the first one is: {:?}", x)
            }
        }
    );

    (l, s, n)

}

/// Generate n random numbers, each one is up to cnt
#[inline]
pub(crate) fn _RAND_IDX(n: impl ToBigUint, cnt: usize) -> Vec<usize> {

    let mut n = n.to_biguint().unwrap();
    let mut idxs = Vec::with_capacity(n.to_usize().unwrap());

    while !n.is_zero() {
        idxs.push(thread_rng().gen_range(0, cnt));
        n -= BigUint::one();
    }

    idxs

}

/// Resolve large numbers into smaller numbers
#[inline]
pub(crate) fn _DIV_UNIT<T>(unit: usize, n: &T) -> Vec<usize>
    where T: Clone + ToBigUint + SubAssign + PartialOrd
{

    let mut n = n.to_biguint().unwrap();

    let UNIT = BigUint::from(unit);
    let mut ret = Vec::with_capacity((&n / &UNIT + BigUint::one()).to_usize().unwrap());

    loop {
        if n < UNIT {
            ret.push(n.to_usize().unwrap());
            break;
        } else {
            n -= UNIT.clone();
            ret.push(unit);
        }
    }

    ret

}

use crate::RandPwd;

/// Generate random password but in the order like "letters->symbols->numbers"
#[inline]
pub(crate) fn _PWD(r_p: &RandPwd) -> String {
    // TODO: - Improve readability

    let unit = r_p._UNIT;
    let data = &DATA;

    vec![(&r_p.ltr_cnt, &data[0]),
         (&r_p.sbl_cnt, &data[1]),
         (&r_p.num_cnt, &data[2]),]
        .iter()
        .map(|(bignum, data)| {
            _DIV_UNIT(unit, *bignum)
                .par_iter()
                .map(|cnt| {
                    _RAND_IDX(*cnt, data.len())
                        .par_iter()
                        // TODO: - Remove this `clone` which can cause huge overhead of both memory and CPU
                        .map(|idx| data[*idx].clone())
                        .collect::<String>()
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat()
        .join("")

}


impl Default for RandPwd {

    #[inline]
    fn default() -> Self {
        RandPwd::new(0, 0, 0)
    }

}


impl Display for RandPwd {

    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\n{}", self.content)
    }

}


impl Add for RandPwd {

    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        RandPwd {
            ltr_cnt: self.ltr_cnt + rhs.ltr_cnt,
            sbl_cnt: self.sbl_cnt + rhs.sbl_cnt,
            num_cnt: self.num_cnt + rhs.num_cnt,
            content: self.content + &rhs.content,
            _UNIT: 1,
        }
    }
}


impl AddAssign for RandPwd {

    #[inline]
    fn add_assign(&mut self, rhs: Self) {

        self.ltr_cnt += rhs.ltr_cnt;
        self.sbl_cnt += rhs.sbl_cnt;
        self.num_cnt += rhs.num_cnt;
        self.content += &rhs.content;

    }
}


impl AsRef<str> for RandPwd {

    #[inline]
    fn as_ref(&self) -> &str {
        &self.content
    }

}


impl From<&str> for RandPwd {

    #[inline]
    fn from(s: &str) -> Self {
        let (ltr_cnt, sbl_cnt, num_cnt) = _CNT(s);
        let mut r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
        r_p.set_val(s);
        r_p.set_unit(1);

        r_p
    }

}


impl<T: AsRef<str>> ToRandPwd for T {

    #[inline]
    fn to_randpwd(&self) -> Option<RandPwd> {
        Some(RandPwd::from(self.as_ref()))
    }

}
