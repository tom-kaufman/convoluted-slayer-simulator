use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    InvalidConfig,
}

struct SlayerTask {
    monster: u32,
    amount: u32,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            MyError::InvalidConfig => { "Invalid configuration!" }
        };
        write!(f, "{}", error_text)
    }
}

// placeholder function, later we'll send a request to localhost:5001
fn slayer_task() -> SlayerTask {
    SlayerTask { monster: 1, amount: 10 }
}


// placeholder function, later we'll send a request to localhost:5002
fn monster_xp(monster: u32) -> u32 {
    assert_eq!(monster, 1);
    return 50;
}

fn total_xp(slayer: &HashMap<u32, (u32, u32)>) -> u32 {
    let mut result = 0;
    for (_, xp) in slayer.values() {
        result += xp;
    }
    result
}

async fn slayer_loop(slayer: &mut HashMap<u32, (u32, u32)>, delta_xp: u32) {
    while total_xp(slayer) < delta_xp {
        let task = slayer_task();
        for _ in 0..task.amount {
            let this_monster = slayer.entry(task.monster).or_insert((0, 0));
            let (mut kills, mut xp) = *this_monster;
            kills += 1;
            xp += monster_xp(task.monster);
        }
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
    let end_xp = 200_000_000_u32;

    // -- end config --

    // check config
    if start_xp >= end_xp {
        return Err(MyError::InvalidConfig);
    }

    // vec: each item corresponds to one of the simulated Slayers
    // hash map: each key corresponds to one of the slayer monsters;
    //           the first u32 in the tuple is the number killed, and
    //           the second rerpresents the total xp 
    let mut v: Vec<Mutex<HashMap<u32, (u32, u32)>>> = vec![];

    // amount of xp each Slayer needs to gain
    let delta_xp = end_xp - start_xp;

    for _ in 0..n {
        v.push(Mutex::new(HashMap::new()));
    }

    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    // for (i, mut slayer) in v.iter().enumerate() {
    let mut i = 0;
    for mut slayer in v {
        println!("moving slayer #{i} to his own thread");
        handles.push(tokio::spawn(async move {
            slayer_loop(slayer.get_mut(), delta_xp).await;
        }));
        println!("finished slayer #{i}!\n");
        i += 1;  // easier than enumerate
    }

    for handle in handles {
        handle.await;
    }

    Ok(())
}
