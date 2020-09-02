
use std::env;
use rand_pwd::RandPwd;
use num_bigint::BigUint;


fn main() {

    let demands = env::args().skip(1).map(|arg| arg.parse::<BigUint>().unwrap()).collect::<Vec<_>>();

    let mut r_p;

    if demands.is_empty() {

        r_p = RandPwd::new(10, 2, 3);
        r_p.join();
        println!("{}", r_p);

    } else {
        
        let ltr_cnt = demands[0].clone();
        let sbl_cnt = demands[1].clone();
        let num_cnt = demands[2].clone();

        let unit;

        if demands.len() == 4 {
            unit = demands[3].clone();
        }

        r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
        r_p.set_unit(unit);
        r_p.join();
        println!("{}", r_p);
    }

}
