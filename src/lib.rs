/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! LKV is a simple collection of key-value containers with linear lookup/search
//! times. Keys and values are stored in simple vectors. The standard library
//! containers should be preferred for applications expecting lots of data,
//! but these containers are potentially preferable for smaller operations.
#![deny(missing_docs)]

/// A one-to-many (OTM) collection stores keys associated with many values.
/// Values may occur more than once for each key.
pub struct OTM<K: Eq, V> {
    inner: Vec<(K, Vec<V>)>,
}

impl<K: Eq, V> OTM<K, V> {
    /// Create a new empty collection.
    ///
    /// ```
    /// # use lkv::OTM;
    /// let otm = OTM::<(), ()>::new();
    /// assert_eq!(otm.num_keys(), 0);
    /// ```
    pub fn new() -> Self {
        Self { inner: Vec::new(), }
    }

    /// Insert a new key-value pair into the collection.
    ///
    /// ```
    /// # use lkv::OTM;
    /// let mut otm = OTM::new();
    /// otm.insert("example", "hello");
    /// otm.insert("example", "world");
    /// assert_eq!(otm.get("example"), &["hello", "world"]);
    /// ```
    pub fn insert(&mut self, key: K, val: V) {
        for bucket in &mut self.inner {
            if &bucket.0 == &key {
                bucket.1.push(val);
                return
            }
        }

        self.inner.push((key, vec![val]));
    }

    /// Get a list of values associated with a key. The returned slice is empty
    /// if no key exists.
    ///
    /// ```
    /// # use lkv::OTM;
    /// let otm = OTM::<&str, ()>::new();
    /// assert_eq!(otm.get("doesn't exist!"), &[]);
    /// ```
    pub fn get(&self, key: K) -> &[V] {
        for bucket in &self.inner {
            if &bucket.0 == &key {
                return &bucket.1;
            }
        }

        return &[];
    }

    /// This is identical to [`Self::get`], but it returns a mutable slice.
    ///
    /// ```
    /// # use lkv::OTM;
    /// let mut otm = OTM::new();
    /// otm.insert(1, 0);
    /// assert_eq!(otm.get(1), &[0]);
    ///
    /// let target = otm.get_mut(1);
    /// target[0] = 1;
    ///
    /// assert_eq!(otm.get(1), &[1]);
    /// ```
    pub fn get_mut(&mut self, key: K) -> &mut [V] {
        for bucket in &mut self.inner {
            if &bucket.0 == &key {
                return &mut bucket.1
            }
        }

        return &mut [];
    }

    /// Insert all `vals` into `self` at `key`, including duplicates.
    ///
    /// ```
    /// # use lkv::OTM;
    /// let mut otm = OTM::new();
    /// otm.insert_many("greeting", vec!["hello", "world"]);
    /// assert_eq!(otm.get("greeting"), &["hello", "world"]);
    /// ```
    pub fn insert_many(&mut self, key: K, vals: Vec<V>) {
        for bucket in &mut self.inner {
            if &bucket.0 == &key {
                bucket.1.extend(vals);
                return;
            }
        }

        self.inner.push((key, vals));
    }

    /// While consuming `other`, insert all of its entries into `self`,
    /// including duplicates.
    pub fn merge(&mut self, other: Self) {
        for (k, vs) in other.inner {
            self.insert_many(k, vs);
        }
    }

    /// Get the number of keys in a collection.
    pub fn num_keys(&self) -> usize {
        self.inner.len()
    }
}
