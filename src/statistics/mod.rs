use document;

//Necessary for hash map
use std::collections::HashMap;
use std::hash::Hash;

pub struct Histogram<EnumType>
where
    EnumType: Eq + Hash,
{
    keys: EnumType,
    value_hashed: HashMap<EnumType, u64>,
}

impl<EnumType> Histogram<EnumType>
where
    EnumType: Eq + Hash,
{
    fn new(initial_keys: EnumType, hash_map: HashMap<EnumType, u64>) -> Histogram<EnumType> {
        return Histogram {
            keys: initial_keys,
            value_hashed: hash_map,
        };
    }

    fn add_value(&mut self, key: EnumType) {
        //Most clone value, or else we will be inserting into the same hashmap (requiring it to be mutable),
        //while we have a value from it (requiring it to be immutable)
        let value = self.value_hashed.get(&key).cloned();
        match value {
            Some(v) => {
                let new_val = v + 1;
                self.value_hashed.insert(key, new_val);
            }
            None => {
                //Don't believe this should happen, but just in case
                self.value_hashed.insert(key, 1);
            }
        }
    }
}

pub struct DocumentStatistics {
    doc: document::Document,
}

impl DocumentStatistics {
    fn new(doc: document::Document) -> DocumentStatistics {
        return DocumentStatistics { doc: doc };
    }

    pub fn memory_transfer_statistics() {}

    pub fn kernel_statistics() {}
}
