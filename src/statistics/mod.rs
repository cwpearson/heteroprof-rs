extern crate gcollections;
extern crate interval;

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

use std::collections::hash_map::Entry;
use std::slice::Iter;

use self::gcollections::ops::*;
use self::interval::interval_set::*;

use callback::Record;

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
        let mut compute_interval_set = vec![(0, 0)].to_interval_set();
        let mut transfer_interval_set = vec![(0, 0)].to_interval_set();

        for compute in self.graph.computes.iter() {
            let (_, compute_val) = compute;
            let compute_start = compute_val.start;
            let compute_end = compute_val.start + compute_val.duration;

            compute_interval_set =
                compute_interval_set.union(&vec![(compute_start, compute_end)].to_interval_set());
        }

        for transfer in self.graph.transfers.iter() {
            let (_, transfer_val) = transfer;
            let transfer_start = transfer_val.start;
            let transfer_end = transfer_val.start + transfer_val.dur;

            transfer_interval_set = transfer_interval_set
                .union(&vec![(transfer_start, transfer_end)].to_interval_set());
        }

        let mut compute_transfer_interval_set = compute_interval_set.clone();
        compute_transfer_interval_set =
            compute_transfer_interval_set.intersection(&transfer_interval_set);

        let mut compute_interval_complement = compute_interval_set.clone();
        let mut transfer_interval_complement = transfer_interval_set.clone();
        compute_interval_complement = compute_interval_complement.complement();
        transfer_interval_complement = transfer_interval_complement.complement();
        compute_interval_set = compute_interval_set.intersection(&transfer_interval_complement);
        transfer_interval_set = transfer_interval_set.intersection(&compute_interval_complement);

        let mut compute_interval_set_count = 0;
        let mut transfer_interval_set_count = 0;
        let mut ct_interval_set_count = 0;
        for interval in compute_interval_set.intervals {
            println!("{:?}", interval);
            compute_interval_set_count += interval.upper() - interval.lower();
        }

        for interval in transfer_interval_set.intervals {
            transfer_interval_set_count += interval.upper() - interval.lower();
        }

        for interval in compute_transfer_interval_set.intervals {
            ct_interval_set_count += interval.upper() - interval.lower();
        }

        println!(
            "Range of compute_interval_set: {}",
            compute_interval_set_count
        );

        println!(
            "Range of transfer_interval_set: {}",
            transfer_interval_set_count
        );

        println!("Range of compute_transfer_set: {}", ct_interval_set_count);
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
}
