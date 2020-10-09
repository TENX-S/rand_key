use rand_key::RandKey;
use std::{env::args, error::Error};




fn main() -> Result<(), Box<dyn Error>> {

    let demands:Vec<String> = args().skip(1).collect();

    let r_p;

    if demands.is_empty() {

        r_p = RandKey::new("10", "2", "3")?;
        r_p.join()?;
        println!("{}", r_p);
    }

    Ok(())

}
