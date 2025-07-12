use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use uuid::Uuid;

use crate::{DynAdapter, EmptyEntity, Entity, EntityAdapter, Error, Rules, evaluate};

/// Which Entity to evaluate?
#[derive(Clone)]
pub struct EvaluateEntity<'a> {
    name: &'a str,
    id: Option<Uuid>,
}

impl<'a> EvaluateEntity<'a> {
    pub fn new(name: &'a str, id: Option<Uuid>) -> Self {
        Self { name, id }
    }
}

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
        A: EntityAdapter + Entity + Default + 'static,
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

    pub async fn load(&self, evaluate: EvaluateEntity<'_>) -> Result<Box<dyn Entity>, Error> {
        let EvaluateEntity {
            name: rsc_name,
            id: rsc_id,
        } = evaluate;

        match rsc_id {
            Some(id) => {
                let rsc_adapter = self.adapters.get(rsc_name).ok_or(Error::AdapterNotFound)?;
                let rsc_provider = self
                    .providers
                    .get(&rsc_adapter.provider_type())
                    .ok_or(Error::ProviderNotFound)?;

                rsc_adapter.load(id, rsc_provider.as_ref()).await
            }
            None => Ok(Box::new(EmptyEntity)),
        }
    }

    pub async fn evaluate(
        &self,
        subject: EvaluateEntity<'_>,
        resource: EvaluateEntity<'_>,
        rules: &Rules,
    ) -> Result<bool, Error> {
        let subject_entity = self.load(subject).await?;
        let resource_entity = self.load(resource).await?;

        evaluate(subject_entity.as_ref(), resource_entity.as_ref(), rules)
    }
}
