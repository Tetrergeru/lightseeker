use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
};

pub trait Id: Eq + Hash + Clone {
    fn empty() -> Self;
    fn next(&self) -> Self;
}

pub struct IdRepositpry<Id, Value> {
    repository: RefCell<HashMap<Id, Value>>,
    last_id: Id,
}

impl<I: Id, Value> IdRepositpry<I, Value> {
    pub fn new() -> Self {
        Self {
            repository: RefCell::new(HashMap::new()),
            last_id: I::empty(),
        }
    }

    pub fn insert(&self, value: Value) -> I {
        let id = self.last_id.next();
        self.repository.borrow_mut().insert(id.clone(), value);
        id
    }

    pub fn remove(&self, id: &I) -> Option<Value> {
        self.repository.borrow_mut().remove(id)
    }

    pub fn get<'a>(&'a self, id: &I) -> Option<&'a Value> {
        let repo = self.repository.borrow();
        if repo.contains_key(id) {
            repo.get(id)
                .map(|it| unsafe { &*(it as *const Value) as &Value })
        } else {
            None
        }
    }

    pub fn get_mut(&self, id: &I) -> Option<RefMut<Value>> {
        let repo = self.repository.borrow_mut();
        if repo.contains_key(id) {
            Some(RefMut::map(repo, |it| it.get_mut(id).unwrap()))
        } else {
            None
        }
    }

    pub fn iter(&self) -> Ref<HashMap<I, Value>> {
        self.repository.borrow()
    }
}

impl<I: Id, Value> Default for IdRepositpry<I, Value> {
    fn default() -> Self {
        Self::new()
    }
}
