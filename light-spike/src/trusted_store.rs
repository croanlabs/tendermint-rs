use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::prelude::*;

pub struct TrustedStore {
    store: HashMap<Height, TrustedState>,
}
impl TrustedStore {
    pub fn get(&self, height: Height) -> Option<&TrustedState> {
        self.store.get(&height)
    }

    pub fn set(&mut self, height: Height, trusted_state: TrustedState) {
        self.store.insert(height, trusted_state);
    }
}

pub struct TSReader {
    ts: Arc<RwLock<TrustedStore>>,
}

impl TSReader {
    pub fn get(&self, height: Height) -> Option<TrustedState> {
        self.ts.read().unwrap().get(height).cloned()
    }
}

pub struct TSReadWriter {
    ts: Arc<RwLock<TrustedStore>>,
}

impl TSReadWriter {
    pub fn get(&self, height: Height) -> Option<TrustedState> {
        self.ts.read().unwrap().get(height).cloned()
    }

    pub fn set(&mut self, height: Height, trusted_state: TrustedState) {
        let mut ts = self.ts.write().unwrap();
        ts.set(height, trusted_state);
    }
}
