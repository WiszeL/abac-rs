use std::{
    any::{Any, TypeId},
    pin::Pin,
};

use uuid::Uuid;

use crate::{Entity, Error};

pub type LoadResult<'a, T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send + 'a>>;

/// Adapter for the entity to load the data before getting evaluated
pub trait EntityAdapter: Entity {
    type Provider: Any + Send + Sync;

    fn load_data(id: Uuid, provider: &Self::Provider) -> LoadResult<Self>
    where
        Self: Sized;
}

pub(crate) trait DynAdapter: Send + Sync {
    /// Which provider does adapter need?
    fn provider_type(&self) -> TypeId;

    /// Load Entity
    fn load<'a>(
        &self,
        id: Uuid,
        provider: &'a (dyn Any + Send + Sync),
    ) -> LoadResult<'a, Box<dyn Entity>>;
}

impl<T> DynAdapter for T
where
    T: EntityAdapter + 'static,
{
    fn provider_type(&self) -> TypeId {
        TypeId::of::<T::Provider>()
    }

    fn load<'a>(
        &self,
        id: Uuid,
        provider: &'a (dyn Any + Send + Sync),
    ) -> LoadResult<'a, Box<dyn Entity>> {
        Box::pin(async move {
            // Get the right provider
            let provider = provider
                .downcast_ref::<T::Provider>()
                .ok_or(Error::ProviderNotFound)?;

            // Load the entity
            let entity = T::load_data(id, provider).await?;

            Ok(Box::new(entity) as Box<dyn Entity>)
        })
    }
}
