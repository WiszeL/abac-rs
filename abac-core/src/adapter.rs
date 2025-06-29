use std::{
    any::{Any, TypeId},
    pin::Pin,
};

use uuid::Uuid;

use crate::Entity;

pub type FutPin<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Adapter for the entity to load the data before getting evaluated
pub trait EntityAdapter: Entity {
    type Provider: Send + Sync;

    fn load_data(id: Uuid, provider: &Self::Provider) -> FutPin<Self>;
}

pub(crate) trait DynAdapter {
    /// Which provider does adapter need?
    fn provider_type(&self) -> TypeId;

    /// Load Entity
    fn load<'a>(
        &self,
        id: Uuid,
        provider: &'a (dyn Any + Send + Sync),
    ) -> FutPin<'a, Box<dyn Entity>>;
}

impl<T> DynAdapter for T
where
    T: EntityAdapter + Send + Sync + 'static,
{
    fn provider_type(&self) -> TypeId {
        TypeId::of::<T::Provider>()
    }

    fn load<'a>(
        &self,
        id: Uuid,
        provider: &'a (dyn Any + Send + Sync),
    ) -> FutPin<'a, Box<dyn Entity>> {
        let provider = provider
            .downcast_ref::<T::Provider>()
            .expect("Provider type mismatch!");

        Box::pin(async move {
            let entity = T::load_data(id, provider).await;

            Box::new(entity) as Box<dyn Entity>
        })
    }
}
