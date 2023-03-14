use std::fmt::Display;
use std::{collections::HashMap, ops::DerefMut, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Debug, Error)]
enum MyError {
    InvalidConfig,
    HandleAwait,
}

struct SlayerTask {
    monster: u32,
    amount: u32,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            MyError::InvalidConfig => "Invalid configuration!",
            MyError::HandleAwait => "Some error happened while waiting for a join handle",
        };
        write!(f, "{}", error_text)
    }
}

// placeholder function, later we'll send a request to localhost:5001
fn slayer_task() -> SlayerTask {
    SlayerTask {
        monster: 1,
        amount: 10,
    }
}

// placeholder function, later we'll send a request to localhost:5002
fn monster_xp(monster: u32) -> u32 {
    assert_eq!(monster, 1);
    50
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
    let this_slayer = slayer_lock.deref_mut();
    while total_xp(this_slayer) < delta_xp {
        let task = slayer_task();
        let mut kills = 0;
        let mut xp = 0;
        for _ in 0..task.amount {
            kills += 1;
            xp += monster_xp(task.monster);
        }
        this_slayer
            .entry(task.monster)
            .and_modify(|(previous_kills, previous_xp)| {
                *previous_kills += kills;
                *previous_xp += xp;
            })
            .or_insert((kills, xp));
    }
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    // -- start config --

    // n: How many Slayers to simulate
    let n = 1_000_u32;
    // initial xp for each Slayer
    let start_xp = 0_u32;
    // final xp for each Slayer
    let end_xp = 13_000_000_u32;

    // -- end config --

    // check config
    if start_xp >= end_xp {
        return Err(MyError::InvalidConfig);
    }

    // amount of xp each Slayer needs to gain
    let delta_xp = end_xp - start_xp;

    #[allow(clippy::type_complexity)]
    let mut slayers: Vec<Arc<Mutex<HashMap<u32, (u32, u32)>>>> = vec![];
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    for i in 0..n {
        println!("moving slayer {i} to his own thread");
        slayers.push(Arc::new(Mutex::new(HashMap::new())));
        let slayer: Arc<Mutex<HashMap<u32, (u32, u32)>>> = Arc::clone(&slayers[i as usize]);
        let handle = tokio::spawn(async move {
            slayer_loop(slayer, delta_xp).await;
            println!("slayer {i} met xp goal!")
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(_) => {}
            Err(_) => { return Err(MyError::HandleAwait); }
        }
    }

    Ok(())
}
