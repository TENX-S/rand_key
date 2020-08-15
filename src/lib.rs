#![allow(non_snake_case)]
#![feature(trait_alias)]

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
    _UNIT: usize,
}


impl RandPwd {

    /// Return an empty instance of `Result<RandPwd, &'static str>`
    /// # Example
    /// ```
    /// use grp::RandPwd;
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
    /// ```
    #[inline]
    pub fn new<T: ToBigUint>(ltr_cnt: T, sbl_cnt: T, num_cnt: T) -> Self {

        RandPwd {
            ltr_cnt: ltr_cnt.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            content: String::new(),
            _UNIT: 1
        }

    }


    /// Generate the password
    #[inline]
    pub fn join(&mut self) {
        let data = &DATA;
        let mut PWD: String = self._PWD((&self.ltr_cnt, &data[0]),
                                        (&self.sbl_cnt, &data[1]),
                                        (&self.num_cnt, &data[2]),);
        let bytes = unsafe { PWD.as_bytes_mut() };
        bytes.shuffle(&mut thread_rng());
        self.content = bytes.par_iter().map(|s| *s as char).collect::<String>();
    }


    /// Return the content of random password in `&str`
    /// # Example
    ///
    /// ```
    /// use grp::RandPwd;
    /// let mut rp = RandPwd::new(10, 2, 3);
    /// rp.join();
    /// println!("{}", rp.show());
    /// // Output: 0fajn-ulS8S}7sn
    /// ```
    #[inline]
    pub fn show(&self) -> &str {
        &self.content
    }


    /// Returns the length of this `RandPwd`, in both bytes and [char]s.
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }


    /// Returns true if this `RandPwd` has a length of zero, and false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }


    /// The value of UNIT is inversely proportional to memory overhead
    /// In order to reduce the memory overhead, raise the value of `UNIT`
    #[inline]
    pub fn set_unit(&mut self, val: usize) {
        self._UNIT = val;
    }


    /// Change the count of letters, symbols or numbers of `RandPwd`
    /// ```
    /// use grp::*;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    ///
    /// // Set the letter's count
    /// r_p.set_cnt("ltr", 0);
    /// r_p.join();
    /// println!("{}", r_p.show());
    /// // Output: *029(
    ///
    /// // Set the symbol's count
    /// r_p.set_cnt("sbl", 0);
    /// r_p.join();
    /// println!("{}", r_p.show());
    /// // Output: nz1MriAl0j5on
    ///
    /// // Set the number's count
    /// r_p.set_cnt("num", 0);
    /// r_p.join();
    /// println!("{}", r_p.show());
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


    /// Generate random password
    #[inline]
    pub(crate) fn _PWD<'a, T: P>(&self, ltr: I<'a, T>, sbl: I<'a, T>, num: I<'a, T>) -> String {
        // TODO: - Improve readability
        vec![(ltr.0, ltr.1),
             (sbl.0, sbl.1),
             (num.0, num.1),]
            .iter()
            .map(|(bignum, data)| {
                self._DIV_UNIT(*bignum)
                    .par_iter()
                    .map(|cnt| {
                        Self::_RAND_IDX(*cnt, data.len())
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


    /// Resolve large numbers into smaller numbers
    #[inline]
    pub(crate) fn _DIV_UNIT<T: P>(&self, n: &T) -> Vec<usize> {

        let mut n = n.to_biguint().unwrap();

        let UNIT = BigUint::from(self._UNIT);
        let mut ret = Vec::with_capacity((&n / &UNIT + BigUint::one()).to_usize().unwrap());

        loop {
            if n < UNIT {
                ret.push(n.to_usize().unwrap());
                break;
            } else {
                n -= UNIT.clone();
                ret.push(self._UNIT);
            }
        }

        ret

    }


    /// Generate n random numbers, each one is up to cnt
    #[inline]
    pub(crate) fn _RAND_IDX(n: impl ToBigUint, cnt: usize) -> Vec<usize> {

        let mut n = n.to_biguint().unwrap();
        let mut idxs = Vec::with_capacity(n.to_usize().unwrap());

        while !n.is_zero() {
            idxs.push(thread_rng().gen_range(0, cnt));
            n -= BigUint::one();
        }

        idxs

    }

}

impl Default for RandPwd {
    fn default() -> Self {
        RandPwd::new(0, 0, 0)
    }
}

impl Display for RandPwd {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.content)
    }
}
