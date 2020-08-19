//! A simple demo of partital API:
//! ```rust
//! use rand_pwd::{ RandPwd, ToRandPwd };

//! fn main() {

//!     let mut r_p = RandPwd::new(10, 2, 3); // For now, it's empty. Use method `join` to generate the password
//!     r_p.join();                           // Now `r_p` has some content, be kept in its `content` field
//!     println!("{}", r_p);                  // Print it on the screen
//!     // One possible output: 7$pA7yMCw=2DPGN

//!     // Or you can build from an existing `&str`
//!     let mut r_p = RandPwd::from("=tE)n5f`sidR>BV"); // 10 letters, 4 symbols, 1 number
//!     // You can rebuild a random password and with equivalent amount of letters, symbols and numbers. Like below
//!     r_p.join();
//!     println!("{}", r_p);
//!     // One possible output: qS`Xlyhpmg~"V8[

//!     // All the `String` and `&str` has implemented trait `ToRandPwd`
//!     // which means you can use method `to_randpwd` to convert a `String` or `&str` to `RandPwd`

//!     let mut r_p = "n4jpstv$dI,.z'K".to_randpwd().unwrap();

//!     // Panic! Has non-ASCII character(s)!
//!     // let mut r_p = RandPwd::from("🦀️🦀️🦀️");
//!     // let mut r_p = "🦀️🦀️🦀️".to_randpwd();
//! }
//! ```

#![allow(non_snake_case)]


#[macro_use]
extern crate lazy_static;

mod prelude;
use prelude::*;


/// struct `RandPwd`
#[derive(Clone, Debug)]
pub struct RandPwd {
    ltr_cnt: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String, // TODO: - use the heapless String
    _UNIT: usize,    // TODO: - implement a smart _UNIT initialization to get best performance
}

/// A generic trait for converting a value to a `RandPwd`.
pub trait ToRandPwd {
    /// Converts the value of `self` to a `RandPwd`.
    fn to_randpwd(&self) -> Option<RandPwd>;
}

impl RandPwd {

    /// Return an empty instance of `Result<RandPwd, &'static str>`
    /// # Example
    /// 
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_bigint::BigUint;
    /// let mut r_p = RandPwd::new(11, 4, 2);
    ///
    /// // If you want push a large number in it
    /// // parse the `&str` into `BigUint`
    /// use std::str::FromStr;
    ///
    /// let ltr_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let sbl_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let num_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    ///
    /// r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
    ///
    /// // You can also mix the `BigUint` with primitive type
    /// ```
    #[inline]
    pub fn new<L, S, N>(ltr_cnt: L, sbl_cnt: S, num_cnt: N) -> Self
    where L: ToBigUint,
          S: ToBigUint,
          N: ToBigUint,
    {

        RandPwd {
            ltr_cnt: ltr_cnt.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            content: String::new(),
            _UNIT: 1
        }

    }


    /// Return the content of random password in `&str`
    /// # Example
    ///
    /// ```
    /// use rand_pwd::RandPwd;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// assert_eq!("", r_p.val())
    /// ```
    #[inline]
    pub fn val(&self) -> &str {
        &self.content
    }


    /// Change the content of `RandPwd`, and updates its fields
    /// # Example
    /// 
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_traits::ToPrimitive;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// r_p.set_val_update("123456");
    /// assert_eq!(r_p.get_cnt("ltr").unwrap().to_usize().unwrap(), 0);
    /// assert_eq!(r_p.get_cnt("sbl").unwrap().to_usize().unwrap(), 0);
    /// assert_eq!(r_p.get_cnt("num").unwrap().to_usize().unwrap(), 6);
    /// ```
    #[inline]
    pub fn set_val_update(&mut self, val: &str) {
        self.ltr_cnt = _CNT(val).0;
        self.sbl_cnt = _CNT(val).1;
        self.num_cnt = _CNT(val).2;
        self.content = val.to_string();
    }

    /// Change the content of `RandPwd`, but check its fields
    /// If it's wrong, it will panic
    /// # Example
    ///
    /// ```
    /// use rand_pwd::RandPwd;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// r_p.set_val_check("123456"); // Will panic
    /// ```
    #[inline]
    pub fn set_val_check(&mut self, val: &str) {
        if (self.ltr_cnt.to_usize().unwrap(),
            self.sbl_cnt.to_usize().unwrap(),
            self.num_cnt.to_usize().unwrap()) == _CNT(val) {
            self.content = val;
        } else {
            panic!("The fields of {:?} is not right", val);
        }
    }


    /// Return the value of `UNIT`
    #[inline]
    pub fn unit(&self) -> usize {
        self._UNIT
    }


    /// The value of UNIT is inversely proportional to memory overhead
    /// In order to reduce the memory overhead, raise the value of `UNIT`
    #[inline]
    pub fn set_unit(&mut self, val: usize) {
        self._UNIT = val;
    }


    /// Returns the length of this `RandPwd`, in both bytes and [char]s.
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// ```
    ///
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }


    /// Returns true if this `RandPwd` has a length of zero, and false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }


    /// Get count of `RandPwd`
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_traits::ToPrimitive;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// assert_eq!(r_p.get_cnt("ltr").unwrap().to_usize().unwrap(), 10);
    /// assert_eq!(r_p.get_cnt("sbl").unwrap().to_usize().unwrap(), 2);
    /// assert_eq!(r_p.get_cnt("num").unwrap().to_usize().unwrap(), 3);
    /// ```
    #[inline]
    pub fn get_cnt(&self, kind: &str) -> Option<&BigUint> {
        match kind {
            "ltr" => Some(&self.ltr_cnt),
            "sbl" => Some(&self.sbl_cnt),
            "num" => Some(&self.num_cnt),

            _   => None,
        }
    }


    /// Change the count of letters, symbols or numbers of `RandPwd`
    /// ```
    /// use rand_pwd::*;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    ///
    /// // Set the letter's count
    /// r_p.set_cnt("ltr", 0);
    /// r_p.join();
    /// println!("{}", r_p.val());
    /// // Output: *029(
    ///
    /// // Set the symbol's count
    /// r_p.set_cnt("sbl", 0);
    /// r_p.join();
    /// println!("{}", r_p.val());
    /// // Output: nz1MriAl0j5on
    ///
    /// // Set the number's count
    /// r_p.set_cnt("num", 0);
    /// r_p.join();
    /// println!("{}", r_p.val());
    /// // Output: +iQiQGSXl(nv
    /// ```
    #[inline]
    pub fn set_cnt<T: ToBigUint>(&mut self, kind: &str, val: T) -> Option<()> {
        match kind {

            "ltr" => self.ltr_cnt = val.to_biguint()?,
            "sbl" => self.sbl_cnt = val.to_biguint()?,
            "num" => self.num_cnt = val.to_biguint()?,

            _     => (),
        }
        Some(())
    }


    /// Generate the password for `RandPwd`
    /// ```
    /// use rand_pwd::RandPwd;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    /// r_p.join();
    /// println!("{}", r_p);
    /// ```
    #[inline]
    pub fn join(&mut self) {
        let mut PWD: String = _PWD(&self);
        // This is absolutely safe, because they are all ASCII characters except control ones.
        let bytes = unsafe { PWD.as_bytes_mut() };
        bytes.shuffle(&mut thread_rng());
        self.content = bytes.par_iter().map(|s| *s as char).collect::<String>();
    }

}
