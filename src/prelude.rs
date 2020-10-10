use {
    std::{
        cell::RefCell,
        fmt::{self, Display, Formatter},
    },
    crate::{
        error::GenError,
        RandKey, ToRandKey,
        SetRandKeyOp::Update,
        utils::{_DEFAULT_DATA, BigUint},
    },
};




pub(crate) const _DEFAULT_UNIT: usize = 2 << 19;


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


impl Default for RandKey {
    /// The default value of `RandKey`
    #[inline]
    fn default() -> Self {
        RandKey {
            ltr_cnt: Default::default(),
            sbl_cnt: Default::default(),
            num_cnt: Default::default(),
            key:     Default::default(),
            UNIT:    RefCell::new(BigUint::from(_DEFAULT_UNIT)),
            DATA:    _DEFAULT_DATA(),
        }
    }
}


impl Display for RandKey {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "\n{}\n", self.key.borrow()) }
}


impl<T: AsRef<str>> ToRandKey for T {
    #[inline]
    fn to_randkey(&self) -> Result<RandKey, GenError> {
        let mut r_p: RandKey = Default::default();
        if r_p.set_key(self.as_ref(), Update).is_ok() {
            Ok(r_p)
        } else {
            Err(GenError::InvalidChar)
        }
    }
}

