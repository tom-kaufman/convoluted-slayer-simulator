# convoluted-slayer-simulator
In the role-playing game RuneScape, Slayer is a skill that is leveled up by getting a "Slayer Task" from a "Slayer Master." Completing a Slayer Task requires killing a particular number of a particular type of monster; each monster killed grants some experience points. 

This project is a monte-carlo simulator for gaining Slayer experience.

The purpose of this project is to learn about asyncio in Rust. 

This project has two simple Flask APIs that can be used to get a slayer task, and to determine how much xp each killed monster grants. Tokio will be used to simulate the tasks.