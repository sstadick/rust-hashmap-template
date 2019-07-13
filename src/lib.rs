use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::borrow::Borrow;

/// Stoped at 1:22:37 of 'Live-coding a linked hash map in Rust'


const INITIAL_NBUCKETS: usize = 1;


pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{

    fn bucket<Q>(&self, key: &Q) -> usize 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        self.items += 1;
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        None

    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V> 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.buckets[self.bucket(key)]
            .iter()
            .find(|&(ref ekey, _)| ekey.borrow() == key)
            .map(|&(_, ref v)| v)
    }


    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key);
        let bucket = &mut self.buckets[bucket];
        let i: usize = bucket.iter().position(|&(ref k, _)| k.borrow() == key)?;
        let (_, v) = bucket.swap_remove(i);
        self.items -= 1;
        Some(v)
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool 
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.is_empty() {
            return false;
        }
        match self.get(key) {
            Some(_) => true,
            None => false,
        }
    }
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket: usize = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        mem::replace(&mut self.buckets, new_buckets);
        // TODO:
    }
}

// Allow iteratation over the HashMap
pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => {
                    match bucket.get(self.at) {
                        Some(&(ref k, ref v)) => {
                            // move along self.at and self.bucket
                            self.at += 1;
                            break Some((k, v));
                        }
                        None => {
                            // move to next bucket
                            self.bucket += 1;
                            self.at = 0;
                            continue;
                        }
                    }
                }
                None => {
                    break None;
                }
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: &self,
            bucket: 0,
            at: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert!(!map.contains_key(&"foo"));
        map.insert("foo", 42);
        assert!(map.contains_key(&"foo"));
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        assert_eq!(map.get(&"foo"), Some(&42));
    }

    #[test]
    fn remove() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.get(&"foo"), None);
    }

    #[test]
    fn iter() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        map.insert("bar", 43);
        map.insert("baz", 142);
        map.insert("quox", 7);

        for (&k, &v) in &map {
            match k {
                "foo" => assert_eq!(v, 42),
                "bar" => assert_eq!(v, 43),
                "baz" => assert_eq!(v, 142),
                "quox" => assert_eq!(v, 7),
                _ => unreachable!(),
            }
        }

        assert_eq!((&map).into_iter().count(), 4);

    }
}
