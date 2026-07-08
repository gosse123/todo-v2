use serde::{Deserialize,Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct Task{
    pub name:String,
    pub description:String,
    pub completed:bool
}
