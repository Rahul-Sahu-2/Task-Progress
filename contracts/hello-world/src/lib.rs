#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Update task progress
    pub fn update_progress(env: Env, task_id: String, progress: i128) {
        let key = symbol_short!("tasks");
        let mut tasks_map: Map<String, i128> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));
        
        tasks_map.set(task_id.clone(), progress);
        env.storage().instance().set(&key, &tasks_map);
    }

    // Get task progress
    pub fn get_progress(env: Env, task_id: String) -> i128 {
        let key = symbol_short!("tasks");
        let tasks_map: Map<String, i128> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));
        
        tasks_map.get(task_id).unwrap_or(0)
    }
}
