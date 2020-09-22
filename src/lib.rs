//! # Usage:
//! ```rust
//! use rand_key::{ RandKey, ToRandKey };
//! fn main() {
//!     let mut r_p = RandKey::new(10, 2, 3); // For now, it's empty. Use method `join` to generate the password
//!     r_p.join();                           // Now `r_p` has some content, be kept in its `key` field
//!     println!("{}", r_p);                  // Print it on the screen
//!     // One possible output: 7$pA7yMCw=2DPGN
//!     // Or you can build from an existing `&str`
//!     let mut r_p = RandKey::from("=tE)n5f`sidR>BV"); // 10 letters, 4 symbols, 1 number
//!     // You can rebuild a random password and with equivalent amount of letters, symbols and numbers. Like below
//!     r_p.join();
//!     println!("{}", r_p);
//!     // One possible output: qS`Xlyhpmg~"V8[
//!     // All the `String` and `&str` has implemented trait `ToRandKey`
//!     // which means you can use method `to_RandKey` to convert a `String` or `&str` to `RandKey`
//!     let mut r_p = "n4jpstv$dI,.z'K".to_randkey();
//!     // Panic! Has non-ASCII character(s)!
//!     // let mut r_p = RandKey::from("🦀️🦀️🦀️");
//!     // let mut r_p = "🦀️🦀️🦀️".to_RandKey();
//! }
//! ```
//! # The `UNIT` field
//! The UNIT field is used to help process large number in concurrent way.
//!
//! If you want to generate a huge random password with 1 million letters, symbols and numbers each,
//! our program will accept such a sequence: [1M, 1M, 1M].
//! However, it takes up huge RAM(Because these numbers are represented in `BigUint`, kind of a `Vec`).
//! And the procedure is single-threaded, you can only process them one by one.
//!
//! The approach is to divide these large numbers into many small numbers,
//! and then process these small numbers in parallel,
//! so the small numbers here can be understood as `UNIT`.
//! For 1M(1 000 000) letters, we set 1K(1000) as the unit value, so [1M] = [1K, 1K, …, 1K] (1000 ones).
//! And we just need to hand this sequence to [rayon](https://github.com/rayon-rs/rayon) for processing.
//! But the disadvantages are also obvious, if `UNIT` number is too small, like `1`,
//! Threads did nothing useful! And capcity of the `Vec` is 1M at least!
//! It will take up huge even all RAM and may harm your computer. **So `RandKey::set_unit()` is unsafe**

#![allow(non_snake_case)]

mod prelude;
mod utils;


use utils::*;




/// struct `RandKey`
#[derive(Clone, Debug)]
pub struct RandKey {
    ltr_cnt: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    key:     String,
    UNIT:    BigUint,
    DATA:    Vec<Vec<String>>,
}


/// A generic trait for converting a value to a `RandKey`.
pub trait ToRandKey {
    /// Converts the value of `self` to a `RandKey`.
    fn to_randkey(&self) -> RandKey;
}


impl RandKey {
    /// Return an empty instance of `Result<RandKey, &'static str>`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    /// use num_bigint::BigUint;
    /// let mut r_p = RandKey::new(11, 4, 2);
    ///
    /// // If you want push a large number in it
    /// // parse the `&str` into `BigUint`
    /// use std::str::FromStr;
    ///
    /// let ltr_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let sbl_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let num_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    ///
    /// r_p = RandKey::new(ltr_cnt, sbl_cnt, num_cnt);
    ///
    /// // You can also mix the `BigUint` with primitive type
    /// ```
    #[inline]
    pub fn new<L, S, N>(ltr_cnt: L, sbl_cnt: S, num_cnt: N) -> Self
        where L: ToBigUint,
              S: ToBigUint,
              N: ToBigUint,
    {
        RandKey {
            ltr_cnt: ltr_cnt.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            key:     String::new(),
            UNIT:    BigUint::from(1024_u16),
            DATA:    _DATA(),
        }
    }

    /// Return the key of random password in `&str`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    ///
    /// let r_p = RandKey::new(10, 2, 3);
    ///
    /// assert_eq!("", r_p.val())
    /// ```
    #[inline]
    pub fn key(&self) -> &str { &self.key }

    /// Change the key of `RandKey`, in the way of the name of operation.
    /// There are two operations: **update** and **check**
    ///
    /// * **update** : Replace the value you've passed and update the field.
    ///
    /// * **check** : If the field of new value doesn't match the old one, it will return an `Err` or the old `key` will be replaced.
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    /// use num_traits::ToPrimitive;
    /// use num_bigint::BigUint;
    ///
    /// // update
    /// let mut r_p = RandKey::new(10, 2, 3);
    ///
    /// assert!(r_p.set_val("123456", "update").is_ok());
    ///
    /// // check
    /// let mut r_p = RandKey::new(10, 2, 3);
    ///
    /// assert!(r_p.set_val("]EH1zyqx3Bl/F8a", "check").is_ok());
    /// assert!(r_p.set_val("123456", "check").is_err());
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn set_key(&mut self, val: &str, op: &str) -> Result<(), String> {
        let (val_ltr_cnt, val_sbl_cnt, val_num_cnt) = _CNT(val);

        match op {
            "update" => {
                self.ltr_cnt = val_ltr_cnt;
                self.sbl_cnt = val_sbl_cnt;
                self.num_cnt = val_num_cnt;
                self.key = val.into();

                Ok(())
            }

            "check" => {
                if (&self.ltr_cnt,
                    &self.sbl_cnt,
                    &self.num_cnt) == (&val_ltr_cnt,
                                       &val_sbl_cnt,
                                       &val_num_cnt) {
                    self.key = val.into();

                    Ok(())
                } else {
                    Err(format!("The fields of {:?} is not right", val))
                }
            }

            _ => Ok(()),
        }
    }

    /// Return the value of `UNIT`
    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use num_traits::One;
    /// use rand_key::RandKey;
    /// use num_bigint::BigUint;
    ///
    /// let r_p = RandKey::new(10, 2, 3); // The default value of unit is 1024
    /// assert_eq!(r_p.unit().clone(), BigUint::from(1024_u16));
    /// ```
    #[inline]
    pub fn unit(&self) -> &BigUint { &self.UNIT }

    /// [set a right `UNIT` number](https://docs.rs/rand_pwd/1.1.3/rand_pwd/#the-unit-field).
    #[inline]
    pub unsafe fn set_unit(&mut self, val: impl ToBigUint) -> Result<(), &str> {
        let val = val.to_biguint().unwrap();

        if val == BigUint::zero() {
            Err("Unit can not be zero!")
        } else {
            self.UNIT = val;

            Ok(())
        }
    }


    /// Return the shared reference of `DATA`
    #[inline]
    pub fn data(&self) -> &Vec<Vec<String>> { &self.DATA }


    /// Clear all the data of `RandPwd`
    #[inline]
    pub fn clear_all(&mut self) { self.DATA.iter_mut().for_each(|x| x.clear()); }

    /// Clear the letters, symbols or numbers
    #[inline]
    pub fn clear(&mut self, kind: &str) {

        match kind {
            "L" => self.DATA[0].clear(),
            "S" => self.DATA[1].clear(),
            "N" => self.DATA[2].clear(),

             _  => (),
        }

    }

    /// Check the data
    #[inline]
    #[allow(non_snake_case)]
    pub fn check_data(&self) -> Result<(), String> {

        let L = self.ltr_cnt.is_zero();
        let S = self.sbl_cnt.is_zero();
        let N = self.num_cnt.is_zero();

        let dl = self.DATA[0].is_empty();
        let ds = self.DATA[1].is_empty();
        let dn = self.DATA[2].is_empty();

        let dl_L = !L && dl;
        let ds_S = !S && ds;
        let dn_N = !N && dn;

        if !(dl_L || ds_S || dn_N) {
            Ok(())
        } else {
            Err("The corresponding character is missing!".into())
        }

    }

    /// Delete the data
    /// # Example
    ///
    /// Basic Usage
    /// ```
    /// use rand_key::RandKey;
    /// ```
    #[inline]
    pub fn delete<T: IntoIterator+Clone>(&mut self, items: T) -> Result<(), String>
        where <T as IntoIterator>::Item: AsRef<str>
    {
        use std::str::FromStr;

        let mut all = self.DATA.concat();

        if check_ascii(items.clone().into_iter()) {

            let mut v = items
                .into_iter()
                .map(|c| char::from_str(c.as_ref()).unwrap())
                .collect::<Vec<_>>();

            v.dedup_by_key(|x| char::clone(x) as u8);

            if  v.iter().skip_while(|x| all.contains(&x.to_string())).next().is_none() {
                all.retain(|x| !v.contains(&char::from_str(x).unwrap()));
                self.DATA = group(all);
            } else {
                panic!("Delete non-exist value!");
            }
        } else {
            panic!("Has non ASCII character(s)");
        }

        self.check_data()

    }

    /// Return a new `RandKey` which has the replaced data
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    /// let mut r_p = RandKey::new(10, 2, 3);
    /// // Missing some kinds of characters will get an Err value
    /// assert!(r_p.replace_data(&["1"]).is_err());
    /// assert!(r_p.replace_data(&["a"]).is_err());
    /// assert!(r_p.replace_data(&["-"]).is_err());
    /// assert!(r_p.replace_data(&["1", "a", "."]).is_ok());
    /// r_p.join();
    /// println!("{}", r_p);
    /// // One possible output: .aa1a1aaaa.a1aa
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn replace_data<T: IntoIterator+Clone>(&mut self, val: T) -> Result<(), String>
        where <T as IntoIterator>::Item: AsRef<str>
    {
        use std::str::FromStr;

        if check_ascii(val.clone().into_iter()) {

            self.DATA = {

                let mut ltr = vec![];
                let mut sbl = vec![];
                let mut num = vec![];

                val.into_iter().for_each(|x| {
                    let x = char::from_str(x.as_ref()).unwrap();

                    if x.is_ascii_alphabetic()  { ltr.push(x.into()); }
                    if x.is_ascii_punctuation() { sbl.push(x.into()); }
                    if x.is_ascii_digit()       { num.push(x.into()); }
                });

                vec![ltr, sbl, num]

            };

            self.check_data()

        } else {
            panic!("Has non ASCII character(s)");
        }
    }

    /// Returns the length of this `RandKey`, in both bytes and [char]s.
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    ///
    /// let mut r_p = RandKey::new(10, 2, 3);
    ///
    /// r_p.join();
    ///
    /// assert_eq!(r_p.len(), 15);
    /// ```
    #[inline]
    pub fn len(&self) -> usize { self.key.len() }

    /// Returns true if this `RandKey` has a length of zero, and false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool { self.key.is_empty() }

    /// Get count of `RandKey`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    ///
    /// use num_traits::ToPrimitive;
    ///
    /// let r_p = RandKey::new(10, 2, 3);
    ///
    /// assert_eq!(r_p.get_cnt("L"), 10.to_biguint());
    /// assert_eq!(r_p.get_cnt("S"), 2.to_biguint());
    /// assert_eq!(r_p.get_cnt("N"), 3.to_biguint());
    /// ```
    #[inline]
    pub fn get_cnt(&self, kind: &str) -> Option<BigUint> {
        match kind {
            "L" => Some(self.ltr_cnt.clone()),
            "S" => Some(self.sbl_cnt.clone()),
            "N" => Some(self.num_cnt.clone()),

            _ => None,
        }
    }

    /// Change the count of letters, symbols or numbers of `RandKey`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::*;
    /// use num_bigint::ToBigUint;
    ///
    /// let mut r_p = RandKey::new(10, 2, 3);
    ///
    /// // Set the letter's count
    /// r_p.set_cnt("L", 20);
    /// assert_eq!(r_p.get_cnt("L"), 10.to_biguint());
    ///
    /// // Set the symbol's count
    /// r_p.set_cnt("S", 1000);
    /// assert_eq!(r_p.get_cnt("S"), 1000.to_biguint());
    ///
    /// // Set the number's count
    /// r_p.set_cnt("N", 0);
    /// assert_eq!(r_p.get_cnt("N"), 0.to_biguint());
    /// ```
    #[inline]
    pub fn set_cnt(&mut self, kind: &str, val: impl ToBigUint) -> Result<(), String> {
        match kind {
            "L" => {
                self.ltr_cnt = val.to_biguint().unwrap();

                Ok(())
            }
            "S" => {
                self.sbl_cnt = val.to_biguint().unwrap();

                Ok(())
            }
            "N" => {
                self.num_cnt = val.to_biguint().unwrap();

                Ok(())
            }

            _ => Err(String::from("No such kind of field in RandKey!")),
        }
    }

    /// Generate the password for `RandKey`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_key::RandKey;
    ///
    /// let mut r_p = RandKey::new(10, 2, 3);
    ///
    /// r_p.join();
    ///
    /// println!("{}", r_p);
    /// ```
    #[inline]
    #[rustfmt::skip]
    pub fn join(&mut self) {

        let mut inner_r_p = self.clone();

        let unit = &inner_r_p.UNIT;
        let data = &inner_r_p.DATA;

        // TODO: - Improve readability
        let mut PWD =
            vec![(&mut inner_r_p.ltr_cnt, &data[0]),
                 (&mut inner_r_p.sbl_cnt, &data[1]),
                 (&mut inner_r_p.num_cnt, &data[2]),]
                .into_iter()
                .map(|(bignum, data)| {
                    _DIV_UNIT(unit, bignum)
                    .par_iter()
                    .map(|cnt| {
                        _RAND_IDX(cnt, data.len())
                            .iter()
                            .map(|idx| data[*idx].clone())
                            .collect::<String>()
                    })
                    .collect()
                })
                .collect::<Vec<Vec<_>>>()
                .concat()
                .join("");

        // This is absolutely safe, because they are all ASCII characters except control ones.
        let bytes = unsafe { PWD.as_bytes_mut() };
        bytes.shuffle(&mut thread_rng());
        self.key = bytes.par_iter().map(|s| *s as char).collect::<String>();
    }

}
