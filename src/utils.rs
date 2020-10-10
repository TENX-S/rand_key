pub use {
    rand::prelude::*,
    rayon::prelude::*,
    num_bigint::{BigUint, ToBigUint},
    num_traits::{Zero, One, ToPrimitive},
};


use {
    crate::error::GenError,
    std::{
        str::FromStr,
        sync::{
            Arc,
            atomic::{Ordering::*, AtomicUsize},
        },
    },
};




/// Characters set
///
/// return letters, symbols, numbers in `Vec<Vec<String>>`
#[inline]
#[rustfmt::skip]
pub(crate) fn _DEFAULT_DATA() -> Vec<Vec<String>> {

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

    let l = Arc::new(AtomicUsize::new(0));
    let s = Arc::new(AtomicUsize::new(0));
    let n = Arc::new(AtomicUsize::new(0));

    content.as_ref()
           .chars()
           .collect::<Vec<_>>()
           .par_iter()
           .for_each(|x| {
               if x.is_ascii() {

                   if x.is_ascii_alphabetic()  {  l.clone().fetch_add(1, SeqCst); }
                   if x.is_ascii_punctuation() {  s.clone().fetch_add(1, SeqCst); }
                   if x.is_ascii_digit()       {  n.clone().fetch_add(1, SeqCst); }

               }
           });

    let l = l.load(SeqCst).to_biguint().unwrap();
    let s = s.load(SeqCst).to_biguint().unwrap();
    let n = n.load(SeqCst).to_biguint().unwrap();

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
pub(crate) fn _CHECK_ASCII(v: &[impl AsRef<str>]) -> bool
{
    v.iter().find(|c| {
            let c = _CHAR_FROM_STR(c);
            !c.is_ascii() || c.is_ascii_control()
        }).is_none()
}


#[inline]
pub(crate) fn _GROUP(v: &[impl AsRef<str>]) -> Vec<Vec<String>> {

    use parking_lot::Mutex;

    let v: Vec<String> = v.iter().map(|x| x.as_ref().to_string()).collect();

    let ltr = Mutex::new(Vec::<String>::new());
    let sbl = Mutex::new(Vec::<String>::new());
    let num = Mutex::new(Vec::<String>::new());

    v.par_iter().for_each(|c| {
        let mut temp;
        let c = _CHAR_FROM_STR(c);

        if c.is_ascii_alphabetic() {
            temp = ltr.lock();
            temp.push(c.clone().into());
        }
        if c.is_ascii_punctuation() {
            temp = sbl.lock();
            temp.push(c.clone().into());
        }
        if c.is_ascii_digit() {
            temp = num.lock();
            temp.push(c.clone().into());
        }
    });

    vec![ltr.into_inner(), sbl.into_inner(), num.into_inner()]
}


#[inline]
pub(crate) fn _CHAR_FROM_STR(s: impl AsRef<str>) -> char { char::from_str(s.as_ref()).unwrap() }
