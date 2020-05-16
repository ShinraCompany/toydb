use super::{Scan, Store};
use crate::error::Result;

use std::collections::BTreeMap;
use std::ops::RangeBounds;

/// In-memory key-value storage using a B-Tree.
pub struct Memory {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Memory {
    /// Creates a new Memory key-value storage engine.
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }
}

impl Store for Memory {
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.data.remove(key);
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.data.get(key).cloned())
    }

    fn scan(&self, range: impl RangeBounds<Vec<u8>>) -> Scan {
        Box::new(self.data.range(range).map(|(k, v)| Ok((k.clone(), v.clone()))))
    }

    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.data.insert(key.to_vec(), value);
        Ok(())
    }
}

#[cfg(test)]
impl super::TestSuite<Memory> for Memory {
    fn setup() -> Result<Self> {
        Ok(Memory::new())
    }
}

#[test]
fn tests() -> Result<()> {
    use super::TestSuite;
    Memory::test()
}