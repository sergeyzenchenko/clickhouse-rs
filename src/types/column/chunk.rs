use std::{cmp, ops};

use crate::{
    binary::Encoder,
    types::{SqlType, Value, ValueRef},
};

use super::{ArcColumnData, ColumnData};

pub struct ChunkColumnData {
    data: ArcColumnData,
    range: ops::Range<usize>,
}

impl ChunkColumnData {
    pub fn new(data: ArcColumnData, range: ops::Range<usize>) -> Self {
        Self { data, range }
    }
}

impl ColumnData for ChunkColumnData {
    fn sql_type(&self) -> SqlType {
        self.data.sql_type()
    }

    fn save(&self, encoder: &mut Encoder, start: usize, end: usize) {
        self.data.save(
            encoder,
            self.range.start + start,
            cmp::min(self.range.end, self.range.start + end),
        )
    }

    fn len(&self) -> usize {
        self.range.len()
    }

    fn push(&mut self, _value: Value) {
        unimplemented!()
    }

    fn at(&self, index: usize) -> ValueRef {
        if index >= self.range.len() {
            panic!("out of range");
        }

        self.data.at(index + self.range.start)
    }
}