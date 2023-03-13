use std::collections::HashMap;
use tokio::sync::RwLock;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    InvalidConfig,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            MyError::InvalidConfig => { "Invalid configuration!" }
        };
        write!(f, "{}", error_text)
    }
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    // -- start config --

    // n: How many Slayers to simulate
    let n = 100_u32;
    // initial xp for each Slayer
    let start_xp = 0_u32;
    // final xp for each Slayer
    let end_xp = 13_100_000_u32;

    // -- end config --

    // check config
    if start_xp >= end_xp {
        return Err(MyError::InvalidConfig);
    }

    // outer map: each key corresponds to one of the simulated Slayers
    // inner map: each key corresponds to one of the slayer monsters;
    //            the first u32 in the tuple is the number killed, and
    //            the second rerpresents the total xp 
    let mut map: HashMap<u32, RwLock<HashMap<u32,(u32, u32)>>> = HashMap::new();

    // amount of xp each Slayer needs to gain
    let delta_xp = end_xp - start_xp;

    for i in 0..n {
        let x = map.entry(i).or_insert(RwLock::new(HashMap::new()));
    }

    println!("Hello, world!");
    Ok(())
}
