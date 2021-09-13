use crate::*;
use core::ops::DerefMut;

pub trait Storage<K, V, E> {
    fn insert(&mut self, key: &K, value: &V) -> Result<Option<V>, E>;
    fn remove(&mut self, key: &K) -> Result<Option<V>, E>;
    fn get(&self, key: &K) -> Result<Option<V>, E>;
    fn contains_key(&self, key: &K) -> Result<bool, E>;
}

impl<K, V, S, I, E> Storage<K, V, E> for I
where
    S: Storage<K, V, E>,
    I: DerefMut<Target = S>,
{
    fn insert(&mut self, key: &K, value: &V) -> Result<Option<V>, E> {
        <S as Storage<K, V, E>>::insert(self.deref_mut(), key, value)
    }

    fn remove(&mut self, key: &K) -> Result<Option<V>, E> {
        <S as Storage<K, V, E>>::remove(self.deref_mut(), key)
    }

    fn get(&self, key: &K) -> Result<Option<V>, E> {
        <S as Storage<K, V, E>>::get(self.deref(), key)
    }

    fn contains_key(&self, key: &K) -> Result<bool, E> {
        <S as Storage<K, V, E>>::contains_key(self.deref(), key)
    }
}

pub trait MerkleStorage<P, K, V, E> {
    fn insert(&mut self, parent: &P, key: &K, value: &V) -> Result<Option<V>, E>;
    fn remove(&mut self, parent: &P, key: &K) -> Result<Option<V>, E>;
    fn get(&self, parent: &P, key: &K) -> Result<Option<V>, E>;
    fn contains_key(&self, parent: &P, key: &K) -> Result<bool, E>;
    fn root(&mut self, parent: &P) -> Result<Bytes32, E>;
}

impl<P, K, V, X, I, E> MerkleStorage<P, K, V, E> for I
where
    X: MerkleStorage<P, K, V, E>,
    I: DerefMut<Target = X>,
{
    fn insert(&mut self, parent: &P, key: &K, value: &V) -> Result<Option<V>, E> {
        <X as MerkleStorage<P, K, V, E>>::insert(self.deref_mut(), parent, key, value)
    }

    fn remove(&mut self, parent: &P, key: &K) -> Result<Option<V>, E> {
        <X as MerkleStorage<P, K, V, E>>::remove(self.deref_mut(), parent, key)
    }

    fn get(&self, parent: &P, key: &K) -> Result<Option<V>, E> {
        <X as MerkleStorage<P, K, V, E>>::get(self.deref(), parent, key)
    }

    fn contains_key(&self, parent: &P, key: &K) -> Result<bool, E> {
        <X as MerkleStorage<P, K, V, E>>::contains_key(self.deref(), parent, key)
    }

    fn root(&mut self, parent: &P) -> Result<Bytes32, E> {
        <X as MerkleStorage<P, K, V, E>>::root(self.deref_mut(), parent)
    }
}
