use std::{
    ops::{Add, AddAssign},
    fmt::{self, Display, Formatter},
};
use crate::{
    error::GenError,
    utils::{_DATA, BigUint},
    RandKey, ToRandKey, SetRandKeyOp::Update,
};




impl Default for RandKey {
    /// The default value of `RandKey`
    #[inline]
    fn default() -> Self {
        RandKey {
            ltr_cnt: Default::default(),
            sbl_cnt: Default::default(),
            num_cnt: Default::default(),
            key:     Default::default(),
            UNIT:    BigUint::from(u16::MAX),
            DATA:    _DATA(),
        }
    }
}


impl Display for RandKey {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "\n{}\n", self.key.borrow()) }
}


impl<T: AsRef<str>> ToRandKey for T {
    type Output = Option<RandKey>;
    #[inline]
    fn to_randkey(&self) -> Self::Output {
        let mut r_p: RandKey = Default::default();
        if r_p.set_key(self.as_ref(), Update).is_ok() {
            Some(r_p)
        } else {
            None
        }
    }
}


pub trait AsBiguint {
    type Output;
    fn as_biguint(&self) -> Self::Output;
}


impl<T: AsRef<str>> AsBiguint for T {
    type Output = Result<BigUint, GenError>;
    #[inline]
    fn as_biguint(&self) -> Self::Output {
        let convert = self.as_ref().parse::<BigUint>();
        if convert.is_ok() { Ok(convert.unwrap()) }
        else { Err(GenError::InvalidNumber) }
    }
}
