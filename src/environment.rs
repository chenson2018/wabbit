use crate::error::{msg, Msg, RangeReporter, Result};
use std::collections::HashMap;

// this allows distinguishing uninitialized types
#[derive(Clone, Debug)]
pub enum VarStore<T> {
    UnInit(T),
    Init(T),
}

impl<T> VarStore<T> {
    pub fn clone_store(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Init(v) | Self::UnInit(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment<'a, T>
where
    T: Clone,
{
    values: HashMap<&'a String, VarStore<T>>,
    parent: Option<Box<Environment<'a, T>>>,
}

impl<'a, T> Environment<'a, T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn in_global_scope(&self) -> bool {
        self.parent.is_none()
    }

    pub fn enter_child(&mut self) {
        *self = Environment {
            values: HashMap::new(),
            parent: Some(box self.clone()),
        }
    }

    pub fn enter_child_fn(&mut self, params: HashMap<&'a String, VarStore<T>>) {
        *self = Environment {
            values: params,
            parent: Some(box self.clone()),
        }
    }

    pub fn exit_child<S>(&mut self, stmt: &S) -> Result<()>
    where
        S: RangeReporter,
    {
        match &self.parent {
            Some(parent) => {
                *self = *parent.clone();
                Ok(())
            }
            None => msg!(
                Msg::InternalErr,
                stmt,
                "Typechecker/Interpreter error, attempted to exit global environment."
            ),
        }
    }

    pub fn exit_child_unwrap(&mut self) {
        *self = *self.parent.clone().unwrap();
    }

    pub fn define_init(&mut self, name: &'a String, value: T) {
        self.values.insert(name, VarStore::Init(value));
    }

    pub fn define_uninit(&mut self, name: &'a String, value: T) {
        self.values.insert(name, VarStore::UnInit(value));
    }

    // up to the caller to determine that we're not changing types!
    pub fn assign(&mut self, name: &'a String, value: T) {
        if self.values.contains_key(&name) {
            self.define_init(name, value)
        } else if let Some(ref mut parent) = self.parent {
            parent.assign(name, value)
        }
    }

    pub fn get(&self, name: &String) -> Option<VarStore<T>> {
        let lookup = self.values.get(name);

        if lookup.is_some() {
            lookup.cloned()
        } else if let Some(ref parent) = self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn top_contains(&self, name: &String) -> bool {
        self.values.contains_key(name)
    }
}
