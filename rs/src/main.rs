use std::{collections::HashMap, sync::Arc, ops::DerefMut};
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

async fn slayer_loop(slayer: Arc<Mutex<HashMap<u32, (u32, u32)>>>, delta_xp: u32) {
    let mut slayer_lock = slayer.lock().await;
    let mut this_slayer = slayer_lock.deref_mut();
    while total_xp(this_slayer) < delta_xp {
        let task = slayer_task();
        for _ in 0..task.amount {
            let this_monster = this_slayer.entry(task.monster).or_insert((0, 0));
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
    let end_xp = 5_u32;

    // -- end config --

    // check config
    if start_xp >= end_xp {
        return Err(MyError::InvalidConfig);
    }

    // amount of xp each Slayer needs to gain
    let delta_xp = end_xp - start_xp;

    let mut slayers: Vec<Arc<Mutex<HashMap<u32, (u32, u32)>>>> = vec![];
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    for i in 0..n {
        slayers.push(Arc::new(Mutex::new(HashMap::new())));
        let slayer: Arc<Mutex<HashMap<u32, (u32, u32)>>> = Arc::clone(&slayers[i as usize]);
        let handle = tokio::spawn(async move {
            slayer_loop(slayer, delta_xp).await;
        });
        handles.push(handle);
    }

    Ok(())
}
