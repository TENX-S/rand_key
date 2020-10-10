use rand_key::RandKey;
use std::{env::args, error::Error};




fn main() -> Result<(), Box<dyn Error>> {
    let demands: Vec<String> = args().skip(1).collect();

    let r_p;

    if demands.is_empty() {
        r_p = RandKey::new("10", "2", "3")?;
        r_p.join()?;
        println!("{}", r_p);
    } else {
        let ltr_cnt = &demands[0];
        let sbl_cnt = &demands[1];
        let num_cnt = &demands[2];

        r_p = RandKey::new(ltr_cnt, sbl_cnt, num_cnt)?;

        if demands.len() == 4 {
            let unit = demands[3].clone();
            r_p.set_unit(unit)?;
        }

        r_p.join()?;

        println!("{}", r_p);
    }

    Ok(())
}
