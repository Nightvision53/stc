#![feature(never_type)]

/// Use good memory allocator.
extern crate swc_node_base;

use std::{
    collections::{HashMap, HashSet},
    env,
};

use once_cell::sync::Lazy;

pub mod cache;
pub mod error;
pub mod ext;
pub mod panic_context;
pub mod stack;

pub type ABuilderHasher = ahash::RandomState;

pub type AHashMap<K, V> = HashMap<K, V, ahash::RandomState>;

pub type AHashSet<V> = HashSet<V, ahash::RandomState>;

/// If true, errors will not be buffered.
pub fn early_error() -> bool {
    static EARLY_ERROR: Lazy<bool> = Lazy::new(|| {
        env::var("STC_EARLY_ERROR")
            .map(|s| s == "1")
            .unwrap_or(false)
    });

    *EARLY_ERROR
}

pub trait TryOpt<T>: Sized + Into<Option<T>> {
    fn try_map<F, U, E>(self, op: F) -> Result<Option<U>, E>
    where
        Self: Into<Option<T>>,
        F: FnOnce(T) -> Result<U, E>,
    {
        match self.into() {
            Some(v) => match op(v) {
                Ok(v) => Ok(Some(v)),
                Err(err) => Err(err),
            },
            None => Ok(None),
        }
    }
}

impl<T> TryOpt<T> for Option<T> {}
