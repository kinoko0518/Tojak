use std::collections::{HashMap, HashSet};

use crate::atom::Atom;

pub struct Agent<'s> {
    /// Atomの参照から実際の状態を返します。
    map: HashMap<&'s str, Atom<'s>>,
}

impl<'s> Agent<'s> {
    pub fn merge(&'s self, other: &'s Self) -> Agent<'s> {
        let keys: HashSet<&str> = self
            .map
            .keys()
            .chain(other.map.keys())
            .map(|k| *k)
            .collect();
        Self {
            map: keys
                .iter()
                .filter_map(|k| {
                    match (self.map.get(k), other.map.get(k)) {
                        (None, None) => None,
                        (Some(me), None) => Some(me.merge_with_nothing()),
                        (None, Some(other)) => Some(other.merge_with_nothing()),
                        (Some(me), Some(other)) => Some(me.merge(other)),
                    }
                    .map(|v| (*k, v))
                })
                .collect(),
        }
    }
}
