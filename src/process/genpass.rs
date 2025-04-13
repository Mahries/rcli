use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-04-03 09:15:29
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-14 02:43:59
 * @FilePath: \rcli\src\process\genpass.rs
 * @Description:
 * Copyright (c) 2025 by Mahires, All Rights Reserved.
 */

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*-_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars: Vec<u8> = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("{}", password);
    let estimate = zxcvbn(&password, &[]);
    eprintln!("Password strength: {:?}", estimate.score());
    Ok(())
}
