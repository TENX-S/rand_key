pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use num_bigint::{BigUint, ToBigUint};
pub use num_traits::{Zero, One, ToPrimitive};


use std::str::FromStr;
use parking_lot::Mutex;
use super::error::GenError;



/// Characters set
///
/// return letters, symbols, numbers in `Vec<Vec<String>>`
#[inline]
#[rustfmt::skip]
pub(crate) fn _DATA() -> Vec<Vec<String>> {

    let mut letters = Vec::new();
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();

    let _ = (33..127).into_iter()
                     .for_each(|x| {
                         let ch = x as u8 as char;

                         if ch.is_ascii_alphabetic()  { letters.push(ch.into()) }
                         if ch.is_ascii_punctuation() { symbols.push(ch.into()) }
                         if ch.is_ascii_digit()       { numbers.push(ch.into()) }
                     });

    vec![letters, symbols, numbers]

}


/// Count the fields of `RandKey` in a string
///
/// The `_CNT("ab123_c53")` returns `(3, 5, 1)`
#[inline]
#[rustfmt::skip]
pub(crate) fn _CNT(content: impl AsRef<str>) -> Result<(BigUint, BigUint, BigUint), GenError> {

    let l = Mutex::new(0);
    let s = Mutex::new(0);
    let n = Mutex::new(0);

    content.as_ref()
           .chars()
           .collect::<Vec<_>>()
           .par_iter()
           .for_each(|x| {
               if x.is_ascii() {
                   let mut temp;

                   if x.is_ascii_alphabetic()  { temp = l.lock(); *temp += 1; }
                   if x.is_ascii_punctuation() { temp = s.lock(); *temp += 1; }
                   if x.is_ascii_digit()       { temp = n.lock(); *temp += 1; }

               }
           });

    let l = l.into_inner().to_biguint().unwrap();
    let s = s.into_inner().to_biguint().unwrap();
    let n = n.into_inner().to_biguint().unwrap();

    if &l+&s+&n != content.as_ref().len().to_biguint().unwrap() {
        Err(GenError::InvalidChar)
    } else {
        Ok((l, s, n))
    }

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

/// Check whether the elements in the sequence are all ascii values
#[inline]
pub(crate) fn check_ascii<T: IntoIterator>(v: T) -> bool
    where <T as IntoIterator>::Item: AsRef<str>
{
    v.into_iter().skip_while(|c| {
        let c = char::from_str(c.as_ref()).unwrap();
        c.is_ascii() && !c.is_ascii_control()
    }).next().is_none()
}


#[inline]
pub(crate) fn group<T: IntoIterator>(v: T) -> Vec<Vec<String>>
    where <T as IntoIterator>::Item: AsRef<str>
{
    let v:Vec<String> = v.into_iter().map(|x| x.as_ref().to_string()).collect();

    let ltr = Mutex::new(Vec::new());
    let sbl = Mutex::new(Vec::new());
    let num = Mutex::new(Vec::new());

    v.par_iter().for_each(|c| {
        let mut temp;
        let c = char::from_str(c).unwrap();

        if c.is_ascii_alphabetic()  { temp = ltr.lock(); temp.push(c.clone().to_string()); }
        if c.is_ascii_punctuation() { temp = sbl.lock(); temp.push(c.clone().to_string()); }
        if c.is_ascii_digit()       { temp = num.lock(); temp.push(c.clone().to_string()); }

    });

    vec![ltr.into_inner(), sbl.into_inner(), num.into_inner()]

}


#[inline]
pub(crate) fn char_from_str(s: impl AsRef<str>) -> char { char::from_str(s.as_ref()).unwrap() }

#[inline]
pub(crate) fn as_biguint(n: impl ToBigUint) -> BigUint { n.to_biguint().unwrap() }
