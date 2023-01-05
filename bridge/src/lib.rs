use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyChildStruct {
    pub v: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyStruct {
    pub tuple: (i32, u32, u64),
    pub string: String,
    pub child: Option<MyChildStruct>,
}
