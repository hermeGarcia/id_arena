use std::ops::Index;
use std::marker::PhantomData;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ArenaId<T> {
    loc: usize,
    content: PhantomData<T>,
}
impl<T> ArenaId<T> {
    pub fn new(loc: usize) -> ArenaId<T> {
        ArenaId {
            loc,
            content: PhantomData
        }
    }
}


pub struct ArenaIter<T> {
    crnt: Option<usize>,
    mark: PhantomData<T>,
    max: usize,
}

impl<T> Iterator for ArenaIter<T> {
    type Item = ArenaId<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.crnt {
            None if self.max != 0 => {
                self.crnt = Some(0);
            },
            Some(v) if v + 1 != self.max => {
                self.crnt = Some(v + 1);
            }
            _ => {
                self.crnt = None;
            }
        }
        self.crnt.map(|v| { ArenaId::new(v) })
    }
}

#[derive(Clone, Default)]
pub struct Arena<T> {
    grains: Vec<T>,
    free: Vec<usize>,
}

impl<T> Index<ArenaId<T>> for Arena<T> {
    type Output = T;
    fn index(&self, index: ArenaId<T>) -> &Self::Output {
        let index_value = index.loc;
        match self.get(index) {
            Some(v) => v,
            None => panic!("Invalid ArenaId used: length is {}, index is {}", self.len(), index_value)
        }
    }
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            grains: vec![],
            free: vec![],
        }
    }

    pub fn alloc(&mut self, value: T) -> ArenaId<T> {
        match self.free.pop() {
            Some(address) => {
                let id = ArenaId::new(address);
                self.grains[address] = value;
                id
            }
            None => {
                let id = ArenaId::new(self.len());
                self.grains.push(value);
                id
            }
        }
    }

    pub fn free(&mut self, ArenaId { loc, .. }: ArenaId<T>) {
        self.free.push(loc);
    }

    pub fn get(&self, ArenaId {loc, ..} : ArenaId<T>) -> Option<&T> {
        if loc >= self.len() {
            None
        } else {
            Some(&self.grains[loc])
        }
    }

    pub fn get_mut(&mut self, ArenaId {loc, .. }: ArenaId<T>) -> Option<&mut T> {
        if loc >= self.len() {
            None
        } else {
            Some(&mut self.grains[loc])
        }
    }

    pub fn iter(&self) -> ArenaIter<T> {
        ArenaIter { crnt: None, mark: PhantomData, max: self.len() }
    }

    pub fn len(&self) -> usize {
        self.grains.len()
    }

    pub fn is_empty(&self) -> bool {
        self.grains.is_empty()
    }
}

#[cfg(test)]
mod arena_test {
    #[test]
    fn arena_index_test() {
        use super::*;
        let mut arena = Arena::new();
        let h_id = arena.alloc("h".to_string());
        let a_id = arena.alloc("a".to_string());
        let l_id = arena.alloc("l".to_string());
        let o_id = arena.alloc("o".to_string());

        assert_eq!("h".to_string(), arena[h_id]);
        assert_eq!("a".to_string(), arena[a_id]);
        assert_eq!("l".to_string(), arena[l_id]);
        assert_eq!("o".to_string(), arena[o_id]);

        let mut word = String::new();
        for id in arena.iter() {
            word.push_str(arena[id].as_str());
        }
        assert_eq!("halo", &word);
    }
    #[test]
    fn arena_empty_test() {
        use super::*;
        let arena: Arena<String> = Arena::new();
        let mut word = String::new();
        for id in arena.iter() {
            word.push_str(arena[id].as_str());
        }
        assert!(word.is_empty());
    }
}
