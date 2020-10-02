
#![allow(non_snake_case)]


#[cfg(test)]
mod InvaidInput {

    use rand_key::RandKey;

    #[test]
    #[should_panic]
    fn test_1() {
        let mut r_p = RandKey::new("A", "B", "C").unwrap();
        r_p.join().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_2() {
        let mut r_p = RandKey::new("-1", "0", "0").unwrap();
        r_p.join().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_3() {
        let mut r_p = RandKey::new("你好", "1", "C").unwrap();
        r_p.join().unwrap();
    }


}


