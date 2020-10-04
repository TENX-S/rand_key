use std::{
    ops::{Add, AddAssign},
    fmt::{self, Display, Formatter},
};
use crate::{
    error::GenError,
    utils::{_DATA, BigUint},
    RandKey, ToRandKey,
    SetRandKeyOp::Update,
};




impl Default for RandKey {
    /// The default value of `RandKey`
    #[inline]
    fn default() -> Self {
        RandKey { ltr_cnt: Default::default(),
                  sbl_cnt: Default::default(),
                  num_cnt: Default::default(),
                  key:     Default::default(),
                  UNIT:    BigUint::from(u16::MAX),
                  DATA:    _DATA(), }
    }
}


impl Display for RandKey {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "\n{}\n", self.key) }
}


impl Add for RandKey {
    type Output = Self;

    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_key::{RandKey, ASCIIExcludeCtrl::*};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut r0 = RandKey::new("1", "2", "3")?;
    /// let mut r1 = RandKey::new("4", "5", "6")?;
    /// let mut r2 = r0 + r1;
    ///
    /// assert_eq!(&r2.get_cnt(Alphabetic), "5");
    /// assert_eq!(&r2.get_cnt(Punctuation), "7");
    /// assert_eq!(&r2.get_cnt(Digit), "9");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn add(self, rhs: Self) -> Self {
        let mut result_data = self.clone();
        result_data.add_item(rhs.DATA.concat()).unwrap();

        RandKey { ltr_cnt: self.ltr_cnt + rhs.ltr_cnt,
                  sbl_cnt: self.sbl_cnt + rhs.sbl_cnt,
                  num_cnt: self.num_cnt + rhs.num_cnt,
                  key:     self.key + &rhs.key,
                  UNIT:    self.UNIT,
                  DATA:    result_data.DATA, }
    }
}


impl AddAssign for RandKey {
    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_key::{RandKey, ASCIIExcludeCtrl::*};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut r0 = RandKey::new("1", "2", "3")?;
    /// let mut r1 = RandKey::new("4", "5", "6")?;
    ///
    /// r0 += r1;
    ///
    /// assert_eq!(&r0.get_cnt(Alphabetic), "5");
    /// assert_eq!(&r0.get_cnt(Punctuation), "7");
    /// assert_eq!(&r0.get_cnt(Digit), "9");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let mut result_data = self.clone();
        result_data.add_item(rhs.DATA.concat()).unwrap();

        self.key += &rhs.key;
        self.ltr_cnt += rhs.ltr_cnt;
        self.sbl_cnt += rhs.sbl_cnt;
        self.num_cnt += rhs.num_cnt;
        self.DATA = result_data.DATA;
    }
}


impl AsRef<str> for RandKey {
    #[inline]
    fn as_ref(&self) -> &str { &self.key }
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
        if let Ok(result) = convert {
            Ok(result)
        } else {
            Err(GenError::InvalidNumber)
        }
    }
}
