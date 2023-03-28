use std::collections::HashMap;
use crate::runtime::{Object, RuntimeObject};

pub struct ObjectStorage {
    object_storage: Vec<HashMap<String, RuntimeObject>>,
    allocation_table: HashMap<usize, u32>
}

impl ObjectStorage {

    pub fn new() -> ObjectStorage {
        ObjectStorage { object_storage: vec![], allocation_table: HashMap::new() }
    }

    pub fn allocate_object(&mut self) -> Object {
        let obj = Object { id: self.get_space() };
        self.inc_reference_count(&obj);
        obj
    }

    pub fn get_field(&self, obj: &Object, name: String) -> Option<RuntimeObject> {
        match self.object_storage[obj.id].get(name.as_str()) {
            Some(a) => Some(a.clone()),
            None => None
        }
    }

    pub fn borow_fields(&self, obj: &Object) -> &HashMap<String, RuntimeObject> {
        &self.object_storage[obj.id]
    }

    pub fn set_field(&mut self, obj: &Object, name: String, value: RuntimeObject) {
        self.object_storage[obj.id].insert(name, value);
    }


    pub fn replace_fields(&mut self, obj: &Object, map: HashMap<String, RuntimeObject>) {
        self.object_storage[obj.id] = map;
    }

    fn check_free_spaces(&self) -> Option<usize> {
        self.allocation_table.iter().filter(|(_, v)| v.clone().clone() == 0).map(|(k, _)| k.clone()).last()
    }

    pub fn inc_reference_count(&mut self, obj: &Object) {
        self.allocation_table.insert(obj.id, self.allocation_table[&obj.id]+1);
    }

    pub fn dec_reference_count(&mut self, obj: &Object) {
        self.allocation_table.insert(obj.id, self.allocation_table[&obj.id]-1);

        //free if no references are held anymore
        if self.allocation_table[&obj.id] == 0 {
            self.object_storage[obj.id].clear()
        }
    }

    fn get_space(&mut self) -> usize {
        match self.check_free_spaces() {
            Some(space) => space,
            None => {
                let u = self.object_storage.len();
                self.object_storage.push(HashMap::new());
                self.allocation_table.insert(u, 0);
                u
            }
        }
    }
}