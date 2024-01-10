use crate::Value;
use werbolg_exec::WAllocator;

pub struct Allocator;

impl WAllocator for Allocator {
    type Value = Value;
}
