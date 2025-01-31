use crate::state::LuauState;
use std::{fmt::Debug, marker::PhantomData};

#[derive(PartialEq)]
pub struct Table<'a> {
    stack_pos: usize,
    _phantom: PhantomData<&'a ()>,
}

impl LuauState {
    pub fn create_table(&self, prealloc_arr: usize, prealloc_map: usize) -> Table {
        todo!()
    }
}

impl<'a> Debug for Table<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Luau Table {:x}>", self.stack_pos)
    }
}
