extern crate serde;
extern crate serde_json;
extern crate petgraph;

use callback;
use cuda::allocation;
use std::rc::Rc;
use cuda::allocation::{AddressSpace, Allocation};
use std;
use std::cmp::{Eq, Ordering, PartialEq};
use std::hash::Hash;
use std::fmt::Debug;
use self::petgraph::graphmap::NodeTrait;


// #[derive(Serialize, Deserialize)]
// struct ValueRaw {
//     value: Value,
// }

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialOrd, Hash)]
pub struct Value {
    pub id: u64,
    pub ptr: u64,
    pub size: u64,
    // pub allocation: Rc<&'a allocation::Allocation<'a>>,
    pub times_modified: u64,
}

// type ValueResult = Result<Value, serde_json::Error>;

// pub fn from_value(v: serde_json::Value) -> ValueResult {
//     let awr: ValueRaw = match serde_json::from_value(v) {
//         Ok(a) => a,
//         Err(e) => return Err(e),
//     };

//     let a = awr.value;

//     Ok(a)
// }

// pub fn val_from_malloc<'a>(
//     v: &callback::CudaMallocS,
//     alloc: &Rc<allocation::Allocation>,
// ) -> Value<'a> {
//     // let awr: Value = match serde_json::from_value(v) {
//     //     Ok(a) => a,
//     //     Err(e) => return Err(e),
//     // };

//     let awr = Value {
//         id: v.id,
//         ptr: v.ptr,
//         size: v.size,
//         allocation: Rc::clone(alloc),
//         times_modified: 1,
//     };

//     awr
// }

//Test no longer applicable
#[test]
fn value_from_malloc_test() {
    // let allocation = Rc::new(Allocation {
    //     id: 0,
    //     pos: 1099882824192,
    //     size: 1024,
    //     address_space: AddressSpace::UVA,
    // });
    // let malloc_s = callback::CudaMallocS {
    //     calling_tid: 11358,
    //     context_uid: 1,
    //     correlation_id: 745,
    //     id: 6,
    //     ptr: 1099882824192,
    //     size: 1024,
    //     symbol_name: std::string::String::from(""),
    //     wall_end: 1522732322549163887,
    //     wall_start: 1522732322549117684,
    // };
    // let b: Value = val_from_malloc(&malloc_s, &allocation);
    // assert_eq!(b.ptr, 1099882824192 as u64);
}
// #[test]
// fn value_test() {
//     use std::io::BufReader;
//     let data = r#"{"allocation":
//                     {"addrsp":{"type":"uva"},
//                     "id":69268689182064,
//                     "loc":{"id":0,"type":"cuda"},
//                     "mem":{"type":"pageable"},
//                     "pos":1099895410688,
//                     "size":2032}
//                 }"#;
//     let mut reader = BufReader::new(data.as_bytes());
//     let v: serde_json::Value = serde_json::from_str(&data).unwrap();
//     let a: Allocation = from_value(v).unwrap();
//     assert_eq!(a.id, 69268689182064 as u64);
// }
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        if (self.id == other.id) {
            return true;
        } else {
            return false;
        }
    }
}

impl Eq for Value {}
