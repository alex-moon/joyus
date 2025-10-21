use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A simple global state container for registering and retrieving shared services/components.
///
/// It stores values keyed by their concrete type using `TypeId`, allowing type-safe retrieval.
#[derive(Default)]
pub struct GlobalState {
    registry: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self { registry: RwLock::new(HashMap::new()) }
    }

    /// Registers a value by its concrete type. If a value of the same type already exists, it is replaced.
    pub fn insert<T>(&self, value: Arc<T>)
    where
        T: Send + Sync + 'static,
    {
        let mut map = self.registry.write().expect("global state poisoned");
        map.insert(TypeId::of::<T>(), value as Arc<dyn Any + Send + Sync>);
    }

    /// Attempts to retrieve a value by its type.
    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        let map = self.registry.read().expect("global state poisoned");
        map.get(&TypeId::of::<T>())
            .and_then(|arc_any| arc_any.clone().downcast::<T>().ok())
    }
}
