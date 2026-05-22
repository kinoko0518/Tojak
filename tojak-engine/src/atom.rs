use std::collections::{HashMap, HashSet};

/// `AtomA.AtomB`のような所有関係を表現します。
pub struct AtomPath<'s> {
    pub owner: &'s str,
    pub owned: &'s str,
}

/// Atomが取りうる状態を表します。
pub enum AtomCondition {
    Some,
    None,
    Superpos,
}

pub struct Atom<'s> {
    having: HashMap<&'s str, AtomCondition>,
}

impl<'s> Atom<'s> {
    pub fn merge(&'s self, other: &'s Self) -> Atom<'s> {
        let keys: HashSet<&str> = self
            .having
            .keys()
            .chain(other.having.keys())
            .map(|k| *k)
            .collect();

        let having = keys
            .iter()
            .map(|key| {
                (
                    *key,
                    match (
                        self.having.get(key).unwrap_or(&AtomCondition::None),
                        other.having.get(key).unwrap_or(&AtomCondition::None),
                    ) {
                        (AtomCondition::None, AtomCondition::None) => AtomCondition::None,
                        (AtomCondition::Some, AtomCondition::Some) => AtomCondition::Some,
                        _ => AtomCondition::Superpos,
                    },
                )
            })
            .collect();

        Self { having }
    }

    pub fn merge_with_nothing(&self) -> Self {
        Self {
            having: self
                .having
                .iter()
                .map(|(k, cond)| {
                    (
                        *k,
                        match cond {
                            AtomCondition::None => AtomCondition::None,
                            _ => AtomCondition::Superpos,
                        },
                    )
                })
                .collect(),
        }
    }
}
