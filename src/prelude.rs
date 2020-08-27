
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };
pub use std::{
    str::FromStr,
    convert::From,
    collections::HashSet,
    ops::{ Add, SubAssign, AddAssign, },
    fmt::{ Display, Formatter, Result, },
};




#[derive(Clone, Debug)]
pub struct Data(Vec<Vec<String>>);


impl Data {

    #[inline]
    pub fn inner(&self) -> &Vec<Vec<String>> {
        &self.0
    }


    #[inline]
    pub fn mut_inner(&mut self) -> &mut Vec<Vec<String>> {
        &mut self.0
    }


    #[inline]
    pub(crate) fn del(&mut self, chs: &[&str]) {

        let mut chs = chs.into_iter().map(|ch| char::from_str(*ch).unwrap()).collect::<Vec<_>>();

        chs.sort();
        chs.dedup();

        for ch in chs {

            if !ch.is_ascii() {
                panic!("Non-ASCII character: {:?}", ch);
            }

            if self.0.concat().contains(&ch.to_string()) {

                let mut idx;

                if ch.is_ascii_alphabetic() {
                    idx = self.0[0].binary_search(&ch.to_string()).unwrap();
                    self.0[0].remove(idx);
                }
                if ch.is_ascii_punctuation() {
                    idx = self.0[1].binary_search(&ch.to_string()).unwrap();
                    self.0[1].remove(idx);
                }
                if ch.is_ascii_digit() {
                    idx = self.0[2].binary_search(&ch.to_string()).unwrap();
                    self.0[2].remove(idx);
                }
            } else {
                panic!("{:?} is not letters, punctuations or digits", ch);
            }
        }
    }

}


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
pub(crate) fn _RAND_IDX(cnt: impl ToBigUint, length: usize) -> Vec<usize> {

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
pub(crate) fn _DIV_UNIT(unit: usize, n: &mut BigUint) -> Vec<usize> {

    let UNIT: BigUint = unit.into();
    let mut ret = Vec::with_capacity((n.clone() / &UNIT + BigUint::one()).to_usize().unwrap());

    loop {
        if n.clone() < UNIT {
            ret.push(n.to_usize().unwrap());
            break;
        } else {
            *n -= UNIT.clone();
            ret.push(unit);
        }
    }

    ret

}

use crate::{ RandPwd, ToRandPwd };


/// Generate random password but in the order like "letters->symbols->numbers"
#[inline]
pub(crate) fn _PWD(r_p: &mut RandPwd) -> String {
    // TODO: - Improve readability

    let unit = r_p._UNIT;
    let data = &r_p._DATA.0;

    vec![(&mut r_p.ltr_cnt, &data[0]),
         (&mut r_p.sbl_cnt, &data[1]),
         (&mut r_p.num_cnt, &data[2]),]
        .into_iter()
        .map(|(bignum, data)| {
            _DIV_UNIT(unit, bignum)
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


impl Default for Data {

    #[inline]
    fn default() -> Self {
        Data(_DATA())
    }

}


impl Display for RandPwd {

    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
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
            _UNIT: 1,
            _DATA: Data::default(),
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
        let (ltr_cnt, sbl_cnt, num_cnt) = _CNT(s);
        let mut r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
        r_p.set_val(s, "update");
        r_p.set_unit(1);

        r_p
    }

}
