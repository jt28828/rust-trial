#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod models;


/* Retrieves damaged structures and creeps in order from most to least damaged */
#[wasm_bindgen]
pub fn get_ordered_damaged_objects(structures: &JsValue, creeps: &JsValue) -> JsValue {
    
    let all_structures: Vec<models::ScreepsStructure> = structures.into_serde().unwrap();
    let my_creeps: Vec<models::ScreepsCreep> = creeps.into_serde().unwrap();

    let damaged_structures: Vec<models::ScreepsStructure> = filter_and_order_structures(all_structures);
    let damaged_creeps: Vec<models::ScreepsCreep> = filter_and_order_creeps(my_creeps);

    let filtered = JsValue::from_serde(&models::SortedScreepsReturn::new(damaged_structures, damaged_creeps)).unwrap();
    return filtered;
}

/** Filters and returns damaged structures in order from most to least damaged */
fn filter_and_order_structures(structures: Vec<models::ScreepsStructure>) -> Vec<models::ScreepsStructure> {
    let mut damaged_structures: Vec<models::ScreepsStructure> = Vec::new();

    for structure in structures.into_iter() {
        if structure.is_damaged() {
            let pos = damaged_structures
                .binary_search_by(|this_struct| this_struct.hits.partial_cmp(&structure.hits).unwrap())
                .unwrap_or_else(|e| e);

            damaged_structures.insert(pos, structure);
        }
    }
    return damaged_structures;
}

/** Filters and returns damaged creeps in order from most to least damaged */
fn filter_and_order_creeps(creeps: Vec<models::ScreepsCreep>) -> Vec<models::ScreepsCreep> {
    let mut damaged_creeps: Vec<models::ScreepsCreep> = Vec::new();
    for structure in creeps.into_iter() {
        if structure.is_damaged() {
            let pos = damaged_creeps
                .binary_search_by(|this_struct| this_struct.hits.partial_cmp(&structure.hits).unwrap())
                .unwrap_or_else(|e| e);

            damaged_creeps.insert(pos, structure);
        }
    }
    return damaged_creeps;
}