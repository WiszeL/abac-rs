use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use uuid::Uuid;

use crate::{DynAdapter, EmptyEntity, Entity, EntityAdapter, Error, Rules, evaluate};

#[derive(Default)]
pub struct Engine {
    pub(crate) adapters: HashMap<&'static str, Box<dyn DynAdapter>>,
    pub(crate) providers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            providers: HashMap::new(),
        }
    }

    #[inline]
    pub fn register_adapter<A>(mut self, name: &'static str) -> Self
    where
        A: EntityAdapter + Entity + Send + Sync + Default + 'static,
    {
        self.adapters.insert(name, Box::<A>::default());

        self
    }

    #[inline]
    pub fn with_provider<P>(mut self, provider: P) -> Self
    where
        P: Any + Send + Sync + 'static,
    {
        self.providers.insert(TypeId::of::<P>(), Box::new(provider));

        self
    }

    pub async fn evaluate_with_subject<S: Entity>(
        &self,
        subject: &S,
        resource: &str,
        resource_id: Option<Uuid>,
        rules: &Rules,
    ) -> Result<bool, Error> {
        let resource_entity = match resource_id {
            Some(id) => {
                let adapter = self.adapters.get(resource).ok_or(Error::AdapterNotFound)?;
                let provider = self
                    .providers
                    .get(&adapter.provider_type())
                    .ok_or(Error::AdapterNotFound)?;

                adapter.load(id, provider.as_ref()).await?
            }
            None => Box::new(EmptyEntity),
        };

        evaluate(subject, resource_entity.as_ref(), rules)
    }
}
