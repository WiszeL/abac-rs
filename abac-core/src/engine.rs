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

    pub async fn evaluate(
        &self,
        subject: EvaluateEntity<'_>,
        resource: EvaluateEntity<'_>,
        rules: &Rules,
    ) -> Result<bool, Error> {
        // Subject
        let EvaluateEntity {
            name: sub_name,
            id: sub_id,
        } = subject;
        let sub_id = sub_id.ok_or(Error::SubjectNotFound)?;
        let sub_adapter = self.adapters.get(sub_name).ok_or(Error::AdapterNotFound)?;
        let sub_provider = self
            .providers
            .get(&sub_adapter.provider_type())
            .ok_or(Error::ProviderNotFound)?;
        let subject_entity = sub_adapter.load(sub_id, sub_provider.as_ref()).await?;

        // Resource
        let EvaluateEntity {
            name: rsc_name,
            id: rsc_id,
        } = resource;
        let resource_entity = match rsc_id {
            Some(id) => {
                let rsc_adapter = self.adapters.get(rsc_name).ok_or(Error::AdapterNotFound)?;
                let rsc_provider = self
                    .providers
                    .get(&rsc_adapter.provider_type())
                    .ok_or(Error::ProviderNotFound)?;

                rsc_adapter.load(id, rsc_provider.as_ref()).await?
            }
            None => Box::new(EmptyEntity),
        };

        evaluate(subject_entity.as_ref(), resource_entity.as_ref(), rules)
    }
}
