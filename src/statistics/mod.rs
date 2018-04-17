/*
Histogram of allocation sizes
Total number of bytes in each bucket

For the kernel launches we should do a histogram of runtimes
Keep track of each unique kernel, where there is a bin per each kernel

Cool to keep track of per device how many kernels are running on there, and how long
they are running for.
 */

use document;

//Necessary for hash map
use std::collections::HashMap;
use std::hash::Hash;

use self::MemoryTransferSizes::*;
use std::slice::Iter;

use callback::Record;

//Talk to Carl what these sizes should be, in terms of what would be most useful for analysis.
#[derive(Debug, Hash, PartialEq, Copy, Clone)]
pub enum MemoryTransferSizes {
    ZeroToFiftyMB,
    FiftyToOneHundredMB,
    OverOneHundredMB,
}

impl Eq for MemoryTransferSizes {}

impl MemoryTransferSizes {
    pub fn iterator() -> Iter<'static, MemoryTransferSizes> {
        static MEMORYTRANSFERSIZES: [MemoryTransferSizes; 3] =
            [ZeroToFiftyMB, FiftyToOneHundredMB, OverOneHundredMB];
        MEMORYTRANSFERSIZES.into_iter()
    }
}

pub struct Histogram<EnumType>
where
    EnumType: Eq + Hash + 'static,
{
    keys: Iter<'static, EnumType>,
    value_hashed: HashMap<EnumType, u64>,
}

impl<EnumType> Histogram<EnumType>
where
    EnumType: Eq + Hash,
{
    fn new(keys: Iter<'static, EnumType>) -> Histogram<EnumType> {
        return Histogram {
            keys: keys,
            value_hashed: HashMap::<EnumType, u64>::new(),
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
    pub fn new(doc: document::Document) -> DocumentStatistics {
        return DocumentStatistics { doc: doc };
    }

    pub fn memory_transfer_statistics(&self) {
        let mut memory_histogram: Histogram<MemoryTransferSizes> =
            Histogram::new(MemoryTransferSizes::iterator());

        for callback_iter in &self.doc.apis {
            match callback_iter {
                &Record::CudaMemcpy(ref s) => {
                    let ref y = s.count;
                    println!("{}", y);
                }
                _ => {
                    //Don't need to do anything, as we are only interested in memory transfers
                }
            }
        }
    }

    pub fn kernel_statistics() {}
}
