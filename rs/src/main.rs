use std::fmt::Display;
use std::ops::Deref;
use std::{collections::HashMap, ops::DerefMut, sync::Arc};
use thiserror::Error;
use tokio::runtime::Builder;
use tokio::sync::Mutex;
use serde::Deserialize;

#[derive(Debug, Error)]
enum MyError {
    InvalidConfig,
    HandleAwait,
    ReqwestSlayerTask,
    ReqwestSlayerTaskJson,
    ReqwestMonsterXp,
    ReqwestMonsterXpJson,
}

#[derive(Deserialize, Debug)]
struct SlayerTask {
    monster: u32,
    amount: u32,
}

#[derive(Deserialize)]
struct SlayerXp {
    xp: f32,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_text = match self {
            MyError::InvalidConfig => "Invalid configuration!",
            MyError::HandleAwait => "Some error happened while waiting for a join handle",
            MyError::ReqwestSlayerTask => "Some error happened while asking for a slayer task",
            MyError::ReqwestSlayerTaskJson => "Some error happened while parsing slayer task json",
            MyError::ReqwestMonsterXp => "Some error happened while asking how much xp to reward for a monster",
            MyError::ReqwestMonsterXpJson => "Some error happened while parsing slayer xp json",
        };
        write!(f, "{}", error_text)
    }
}

async fn slayer_task() -> Result<SlayerTask, MyError> {
    let resp = match reqwest::get("http://127.0.0.1:5001/").await {
        Ok(x) => { x }
        Err(_) => { return Err(MyError::ReqwestSlayerTask); }
    };
    let j = match resp.json::<SlayerTask>().await {
        Ok(x) => { x}
        Err(e) => { return Err(MyError::ReqwestSlayerTaskJson); }
    };
    Ok(j)
}

async fn monster_xp(monster: u32) -> Result<f32, MyError> {
    let url = format!("http://127.0.0.1:5002/{monster}");
    let resp = match reqwest::get(url).await {
        Ok(x) => { x }
        Err(_) => { return Err(MyError::ReqwestMonsterXp); }
    };
    match resp.json::<SlayerXp>().await {
        Ok(x) => { Ok(x.xp) }
        Err(e) => { return Err(MyError::ReqwestMonsterXpJson); }
    }
}

fn total_xp(slayer: &HashMap<u32, (u32, f32)>) -> f32 {
    let mut result = 0.;
    for (_, xp) in slayer.values() {
        result += xp;
    }
    result
}

async fn slayer_loop(slayer: Arc<Mutex<HashMap<u32, (u32, f32)>>>, delta_xp: f32) -> Result<(), MyError> {
    let mut slayer_lock = slayer.lock().await;
    let this_slayer = slayer_lock.deref_mut();
    while total_xp(this_slayer) < delta_xp {
        let task = slayer_task().await?;
        let mut kills = 0;
        let mut xp = 0.;
        for _ in 0..task.amount {
            kills += 1;
            xp += monster_xp(task.monster).await?;
        }
        this_slayer
            .entry(task.monster)
            .and_modify(|(previous_kills, previous_xp)| {
                *previous_kills += kills;
                *previous_xp += xp;
            })
            .or_insert((kills, xp));
    }
    Ok(())
}


fn main() -> Result<(), MyError> {
    // -- start config --

    // n: How many Slayers to simulate
    let n = 10_u32;
    // initial xp for each Slayer
    let start_xp = 0_f32;
    // final xp for each Slayer
    let end_xp = 200_000_000_f32;

    // -- end config --

    // check config
    if start_xp >= end_xp {
        return Err(MyError::InvalidConfig);
    }

    // amount of xp each Slayer needs to gain
    let delta_xp = end_xp - start_xp;

    #[allow(clippy::type_complexity)]
    let mut slayers: Vec<Arc<Mutex<HashMap<u32, (u32, f32)>>>> = vec![];
    let mut handles: Vec<tokio::task::JoinHandle<Result<(), MyError>>> = vec![];

    // create tokio runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // our async part of the mainloop
    runtime.block_on(async {
        for i in 0..n {
            println!("moving slayer {i} to his own thread");
            slayers.push(Arc::new(Mutex::new(HashMap::new())));
            let slayer: Arc<Mutex<HashMap<u32, (u32, f32)>>> = Arc::clone(&slayers[i as usize]);
            let handle = tokio::spawn(async move {
                match slayer_loop(slayer, delta_xp).await {
                    Ok(o) => {  }
                    Err(e) => { return Err(e) }
                }
                println!("slayer {i} met xp goal!");
                Ok(())
            });
            handles.push(handle);
        }
    
        for handle in handles {
            match handle.await {
                Ok(_) => {}
                Err(_) => {
                    return Err(MyError::HandleAwait);
                }
            }
        }

        Ok(())
    })?;

    // Unpack slayers from Arc<Mutex<T>>
    let slayers_convenient = slayers
        .iter()
        .map(|slayer| {
            let slayer_lock = runtime.block_on(slayer.lock());
            let x = slayer_lock.deref();
            x.clone()
        })
        .collect::<Vec<HashMap<u32, (u32, f32)>>>();

    for (i, slayer) in slayers_convenient.iter().enumerate() {
        println!("slayer {i}:\n{:?}\n\n", slayer);
    }

    Ok(())
}
