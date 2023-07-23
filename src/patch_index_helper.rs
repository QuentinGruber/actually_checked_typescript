pub struct ModifiedIndex {
    pub pos: u32,
    pub size: u32,
}

pub struct PatchIndexHelper {
    pub higher_index: u32,
    pub indexes_modified: Vec<ModifiedIndex>,
}

impl PatchIndexHelper {
    pub fn new() -> PatchIndexHelper {
        PatchIndexHelper {
            higher_index: 0,
            indexes_modified: vec![],
        }
    }

    pub fn register_patched_index(&mut self, index: u32, size: u32) {
        let modified_index = ModifiedIndex { pos: index, size };
        self.indexes_modified.push(modified_index);
    }

    pub fn get_drifted_index(&mut self, original_index: u32) -> u32 {
        let mut drifted_index = original_index;
        for index_modify in &self.indexes_modified {
            if original_index >= index_modify.pos {
                drifted_index += index_modify.size;
            }
        }
        drifted_index
    }
}

