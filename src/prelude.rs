
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };
pub use std::{
    str::FromStr,
    ops::{ Add, SubAssign, AddAssign, },
    fmt::{ self, Display, Formatter, },
};




/// Characters set
///
/// return letters, symbols, numbers in `CharVec`
#[inline]
pub(crate) fn _DATA() -> Vec<Vec<String>> {

    let mut letters = Vec::<String>::new();
    let mut symbols = Vec::<String>::new();
    let mut numbers = Vec::<String>::new();

    let mut charset = vec![];

    let _ = (33..127)
            .into_iter()
            .map(|x| {
                let ch = x as u8 as char;
                if ch.is_ascii_alphabetic()  { letters.push(ch.into()); }
                if ch.is_ascii_punctuation() { symbols.push(ch.into()); }
                if ch.is_ascii_digit()       { numbers.push(ch.into()); }
            })
            .collect::<()>();

    charset.push(letters);
    charset.push(symbols);
    charset.push(numbers);

    charset

}


/// Count the number of a string
#[inline]
pub(crate) fn _CNT<T: AsRef<str>>(content: T) -> (BigUint, BigUint, BigUint) {

    use std::sync::Mutex;

    let l = Mutex::new(0);
    let s = Mutex::new(0);
    let n = Mutex::new(0);

    content.as_ref().chars().collect::<Vec<_>>().par_iter().for_each(
        |x| {
            if x.is_ascii() {
                if x.is_ascii_alphabetic()  {
                    let mut temp = l.lock().unwrap();
                    *temp += 1;
                }
                if x.is_ascii_punctuation() {
                    let mut temp = s.lock().unwrap();
                    *temp += 1;
                }
                if x.is_ascii_digit()       {
                    let mut temp = n.lock().unwrap();
                    *temp += 1;
                }
            } else {
                panic!("Has non-ASCII character(s)!, the first one is: {:?}", x)
            }
        }
    );

    (l.into_inner().unwrap().to_biguint().unwrap(),
     s.into_inner().unwrap().to_biguint().unwrap(),
     n.into_inner().unwrap().to_biguint().unwrap(),)

}


/// Generate n random numbers, each one is up to `length`
#[inline]
pub(crate) fn _RAND_IDX(cnt: &BigUint, length: usize) -> Vec<usize> {

    let mut n = cnt.to_biguint().unwrap();
    let mut idxs = Vec::with_capacity(n.to_usize().unwrap());

    while !n.is_zero() {
        idxs.push(thread_rng().gen_range(0, length));
        n -= BigUint::one();
    }

    idxs

}


/// Resolve large numbers into smaller numbers
#[inline]
pub(crate) fn _DIV_UNIT(unit: &BigUint, n: &mut BigUint) -> Vec<BigUint> {

    let UNIT = unit.to_biguint().unwrap();
    let mut ret = Vec::with_capacity((n.clone() / &UNIT + BigUint::one()).to_usize().unwrap());

    loop {
        if n.clone() < UNIT {
            ret.push(n.to_biguint().unwrap());
            break;
        } else {
            *n -= UNIT.clone();
            ret.push(UNIT.clone());
        }
    }

    ret

}

use crate::{ RandPwd, ToRandPwd };


impl Default for RandPwd {

    #[inline]
    fn default() -> Self {
        RandPwd {
            ltr_cnt: Default::default(),
            sbl_cnt: Default::default(),
            num_cnt: Default::default(),
            content: Default::default(),
            UNIT: BigUint::one(),
            DATA: _DATA(),
        }
    }

}


// impl Default for Data {
//
//     #[inline]
//     fn default() -> Self {
//         Data(_DATA())
//     }
//
// }


impl Display for RandPwd {

    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\n{}\n", self.content)
    }

}


impl Add for RandPwd {

    type Output = Self;

    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_bigint::BigUint;
    /// let mut r0 = RandPwd::new(1, 2, 3);
    /// let mut r1 = RandPwd::new(4, 5, 6);
    /// let mut r2 = r0 + r1;
    ///
    /// assert_eq!(*r2.get_cnt("ltr").unwrap(), BigUint::from(5_usize));
    /// assert_eq!(*r2.get_cnt("sbl").unwrap(), BigUint::from(7_usize));
    /// assert_eq!(*r2.get_cnt("num").unwrap(), BigUint::from(9_usize));
    ///
    /// ```
    #[inline]
    fn add(self, rhs: Self) -> Self {
        RandPwd {
            ltr_cnt: self.ltr_cnt + rhs.ltr_cnt,
            sbl_cnt: self.sbl_cnt + rhs.sbl_cnt,
            num_cnt: self.num_cnt + rhs.num_cnt,
            content: self.content + &rhs.content,
            UNIT: self.UNIT,
            DATA: Default::default(),
        }
    }
}


impl AddAssign for RandPwd {

    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_bigint::BigUint;
    /// let mut r0 = RandPwd::new(1, 2, 3);
    /// let mut r1 = RandPwd::new(4, 5, 6);
    /// r0 += r1;
    /// assert_eq!(*r0.get_cnt("ltr").unwrap(), BigUint::from(5_usize));
    /// assert_eq!(*r0.get_cnt("sbl").unwrap(), BigUint::from(7_usize));
    /// assert_eq!(*r0.get_cnt("num").unwrap(), BigUint::from(9_usize));
    ///
    /// ```
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


impl<T: AsRef<str>> ToRandPwd for T {

    #[inline]
    fn to_randpwd(&self) -> Option<RandPwd> {
        Some(self.as_ref().into())
    }

}


impl From<&str> for RandPwd {

    #[inline]
    fn from(s: &str) -> Self {
        let mut r_p = RandPwd::default();
        r_p.set_val(s, "update");

        r_p

    }

}
