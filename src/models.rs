#[derive(Deserialize)]
pub struct ScreepsRoom {
    pub controller: ScreepsStructure
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ScreepsStructure {
    pub hits: i32,
    pub hitsMax: i32,
}

impl ScreepsStructure {
    pub fn is_damaged(&self) -> bool {
        return self.hitsMax < self.hitsMax;
    }
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ScreepsCreep {
    pub hits: i32,
    pub hitsMax: i32,
}

impl ScreepsCreep {
    pub fn is_damaged(&self) -> bool {
        return self.hitsMax < self.hitsMax;
    }
}

#[derive(Serialize)]
pub struct SortedScreepsReturn {
    pub orderedStructures: Vec<ScreepsStructure>,
    pub orderedCreeps: Vec<ScreepsCreep>,
}

impl SortedScreepsReturn {
    pub const fn new(structures: Vec<ScreepsStructure>, creeps: Vec<ScreepsCreep>) -> SortedScreepsReturn {
        SortedScreepsReturn {
            orderedStructures: structures,
            orderedCreeps: creeps,
        }
    }
}