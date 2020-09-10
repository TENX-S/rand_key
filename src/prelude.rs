

use crate::{ RandPwd, ToRandPwd, utils::{ One, _DATA, BigUint, } };
use std::{ ops::{ Add, AddAssign, }, fmt::{ self, Display, Formatter, }, };




impl Default for RandPwd {

    /// The default value of `RandPwd`
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
    fn to_randpwd(&self) -> RandPwd {
        self.as_ref().into()
    }

}


impl From<&str> for RandPwd {

    #[inline]
    fn from(s: &str) -> Self {
        let mut r_p = RandPwd::default();
        r_p.set_val(s, "update").unwrap();

        r_p

    }

}
