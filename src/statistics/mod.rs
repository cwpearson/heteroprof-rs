/*
Histogram of allocation sizes
Total number of bytes in each bucket

For the kernel launches we should do a histogram of runtimes
Keep track of each unique kernel, where there is a bin per each kernel

Cool to keep track of per device how many kernels are running on there, and how long
they are running for.
 */

use document;
use pdg::pdg::PDG;

//Necessary for hash map
use pdg::transfer::Transfer;
use std::collections::HashMap;
use std::hash::Hash;

// use self::MemoryTransferSizes::*;
use std::collections::hash_map::Entry;
use std::slice::Iter;

use callback::Record;
/* Old code for histogramming
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
}*/

#[derive(Debug, Hash, PartialEq, Eq)]
enum BinOverLapTypes {
    ComputeOnly,
    TransferOnly,
    ComputeTransfer,
}

pub struct DocumentStatistics<'a> {
    graph: &'a mut PDG<'a>,
    overlap_bins: HashMap<BinOverLapTypes, u64>,
}

impl<'a> DocumentStatistics<'a> {
    pub fn new(pdg: &'a mut PDG<'a>) -> DocumentStatistics<'a> {
        return DocumentStatistics::<'a> {
            graph: pdg,
            overlap_bins: HashMap::new(),
        };
    }

    pub fn generate_bins(&mut self) {
        let mut compute_transfer = 0;
        let mut compute_only = 0;
        let mut transfer_only = 0;
        {
            let mut transfer_iter = self.graph.transfers.iter();
            let mut peeked_transfer = transfer_iter.next();

            for compute in self.graph.computes.iter() {
                let (_, compute_val) = compute;
                let compute_start = compute_val.start;
                let compute_end = compute_start + compute_val.duration;

                match peeked_transfer {
                    Some(v) => {
                        let (_, transfer_value) = v;
                        let transfer_start_time = transfer_value.start;
                        let transfer_duration = transfer_value.dur;
                        let transfer_end = transfer_start_time + transfer_duration;
                        println!(
                            "Compute Start: {}; Compute End: {}",
                            compute_start, compute_end
                        );

                        println!(
                            "Transfer Start: {}; Transfer End: {}",
                            transfer_start_time, transfer_end
                        );

                        //Check to see the current overlap
                        if transfer_start_time < compute_start && transfer_end > compute_end {
                            compute_transfer += 1;
                        } else if transfer_start_time > compute_start
                            && transfer_start_time < compute_end
                        {
                            compute_transfer += 1;
                        } else if transfer_end < compute_end {
                            compute_transfer += 1;
                            peeked_transfer = transfer_iter.next();
                            let mut condition = self.check_recursive(peeked_transfer, compute_end);
                            while condition {
                                compute_transfer += 1;
                                peeked_transfer = transfer_iter.next();
                                condition = self.check_recursive(peeked_transfer, compute_end)
                            }
                        } else if transfer_start_time > compute_end {
                            println!("Transfer Start after Compute end");

                            compute_only += 1;
                        } else if transfer_end < compute_start {
                            println!("Transfer End before Compute Start");
                            transfer_only += 1;
                            peeked_transfer = transfer_iter.next();
                        }
                    }
                    _ => {
                        compute_only += 1;
                    }
                }
            }
        }

        self.set_bins(compute_only, transfer_only, compute_transfer);
    }

    fn set_bins(&mut self, compute_only: u64, transfer_only: u64, compute_transfer: u64) {
        self.overlap_bins
            .insert(BinOverLapTypes::ComputeTransfer, compute_transfer);
        self.overlap_bins
            .insert(BinOverLapTypes::ComputeOnly, compute_only);
        self.overlap_bins
            .insert(BinOverLapTypes::TransferOnly, transfer_only);

        println!(
            "Compute Only: {} ; Transfer Only: {} ; Compute Transfer: {}",
            compute_only, transfer_only, compute_transfer
        );
    }

    fn check_recursive(&self, transfer: Option<(&u64, &Transfer)>, compute_end: u64) -> bool {
        let val = match transfer {
            Some(v) => {
                let (_, transfer_value) = v;
                let transfer_start_time = transfer_value.start;
                let transfer_duration = transfer_value.dur;
                let transfer_end = transfer_start_time + transfer_duration;
                if transfer_end < compute_end {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        val
    }

    fn compute_only_bin(&mut self) {
        match self.overlap_bins.entry(BinOverLapTypes::ComputeOnly) {
            Entry::Occupied(ent) => {
                let ent_mut = ent.into_mut();
                *ent_mut += 1;
            }
            Entry::Vacant(ent) => {
                ent.insert(1);
            }
        }
    }

    fn transfer_only_bin(&mut self) {
        match self.overlap_bins.entry(BinOverLapTypes::TransferOnly) {
            Entry::Occupied(ent) => {
                let ent_mut = ent.into_mut();
                *ent_mut += 1;
            }
            Entry::Vacant(ent) => {
                ent.insert(1);
            }
        }
    }

    fn compute_transfer_bin(&mut self) {
        match self.overlap_bins.entry(BinOverLapTypes::ComputeTransfer) {
            Entry::Occupied(ent) => {
                let ent_mut = ent.into_mut();
                *ent_mut += 1;
            }
            Entry::Vacant(ent) => {
                ent.insert(1);
            }
        }
    }

    //Old histogram generation code
    // pub fn memory_transfer_statistics(&self) {
    //     let memory_histogram: Histogram<MemoryTransferSizes> =
    //         Histogram::new(MemoryTransferSizes::iterator());

    //     for callback_iter in &self.doc.apis {
    //         match callback_iter {
    //             &Record::CudaMemcpy(ref s) => {
    //                 let ref y = s.count;
    //                 println!("Memory transfer!: {}", y);
    //             }
    //             _ => {
    //                 //Don't need to do anything, as we are only interested in memory transfers
    //             }
    //         }
    //     }
    // }

    // pub fn kernel_statistics() {}
}
