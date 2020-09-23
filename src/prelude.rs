use crate::{
    RandKey, ToRandKey,
    utils::{_DATA, BigUint},
};

use std::{
    ops::{Add, AddAssign},
    fmt::{self, Display, Formatter},
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "\n{}\n", self.key) }
}


impl Add for RandKey {
    type Output = Self;

    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_key::RandKey;
    /// use num_bigint::{BigUint, ToBigUint};
    ///
    /// let mut r0 = RandKey::new(1, 2, 3);
    /// let mut r1 = RandKey::new(4, 5, 6);
    /// let mut r2 = r0 + r1;
    ///
    /// assert_eq!(r2.get_cnt("L"), 5.to_biguint());
    /// assert_eq!(r2.get_cnt("S"), 7.to_biguint());
    /// assert_eq!(r2.get_cnt("N"), 9.to_biguint());
    /// ```
    #[inline]
    fn add(self, rhs: Self) -> Self {
        RandKey {
            ltr_cnt: self.ltr_cnt + rhs.ltr_cnt,
            sbl_cnt: self.sbl_cnt + rhs.sbl_cnt,
            num_cnt: self.num_cnt + rhs.num_cnt,
            key:     self.key     + &rhs.key,
            UNIT:    self.UNIT,
            DATA:    Default::default(),
        }
    }
}


impl AddAssign for RandKey {
    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_key::RandKey;
    /// use num_bigint::BigUint;
    ///
    /// let mut r0 = RandKey::new(1, 2, 3);
    /// let mut r1 = RandKey::new(4, 5, 6);
    ///
    /// r0 += r1;
    ///
    /// assert_eq!(r0.get_cnt("L").unwrap(), BigUint::from(5_usize));
    /// assert_eq!(r0.get_cnt("S").unwrap(), BigUint::from(7_usize));
    /// assert_eq!(r0.get_cnt("N").unwrap(), BigUint::from(9_usize));
    /// ```
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.ltr_cnt += rhs.ltr_cnt;
        self.sbl_cnt += rhs.sbl_cnt;
        self.num_cnt += rhs.num_cnt;
        self.key     += &rhs.key;
    }
}


impl AsRef<str> for RandKey {
    #[inline]
    fn as_ref(&self) -> &str { &self.key }
}


impl<T: AsRef<str>> ToRandKey for T {
    #[inline]
    fn to_randkey(&self) -> RandKey { self.as_ref().into() }
}


impl From<&str> for RandKey {
    #[inline]
    fn from(s: &str) -> Self {
        let mut r_p = RandKey::default();
        r_p.set_key(s, "update").unwrap();

        r_p

    }
}
