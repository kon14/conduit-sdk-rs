mod error;
mod inner;

pub use error::ModuleRegistryError;
use inner::InnerModuleRegistry;

use crate::modules::Module;
use crate::sd::ModuleName;
use std::fmt;
use std::sync::{Arc, RwLock};

/// A locally tracked registry of connected modules.
#[derive(Default)]
pub struct ModuleRegistry(Arc<RwLock<InnerModuleRegistry>>);

impl ModuleRegistry {
    pub fn add_module(&self, name: ModuleName, module: Module) -> Result<(), ModuleRegistryError> {
        let mut registry = self.0.write().unwrap();
        registry.add_module(name, module)
    }

    pub fn remove_module(&self, name: ModuleName) -> Result<Module, ModuleRegistryError> {
        let mut registry = self.0.write().unwrap();
        registry.remove_module(name)
    }

    pub fn get_module(&self, name: &ModuleName) -> Option<Module> {
        let registry = self.0.read().unwrap();
        registry.get_module(name)
    }

    pub fn is_registered(&self, name: &ModuleName) -> bool {
        let registry = self.0.read().unwrap();
        registry.is_registered(name)
    }

    pub fn get_authentication(
        &self,
    ) -> Result<Arc<crate::modules::authentication::AuthenticationModule>, ModuleRegistryError>
    {
        let registry = self.0.read().unwrap();
        registry.get_authentication()
    }

    pub fn get_authorization(
        &self,
    ) -> Result<Arc<crate::modules::authorization::AuthorizationModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_authorization()
    }

    pub fn get_chat(&self) -> Result<Arc<crate::modules::chat::ChatModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_chat()
    }

    pub fn get_database(
        &self,
    ) -> Result<Arc<crate::modules::database::DatabaseModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_database()
    }

    pub fn get_email(
        &self,
    ) -> Result<Arc<crate::modules::email::EmailModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_email()
    }

    pub fn get_forms(
        &self,
    ) -> Result<Arc<crate::modules::forms::FormsModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_forms()
    }

    pub fn get_push_notifications(
        &self,
    ) -> Result<Arc<crate::modules::push_notifications::PushNotificationsModule>, ModuleRegistryError>
    {
        let registry = self.0.read().unwrap();
        registry.get_push_notifications()
    }

    pub fn get_router(
        &self,
    ) -> Result<Arc<crate::modules::router::RouterModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_router()
    }

    pub fn get_sms(&self) -> Result<Arc<crate::modules::sms::SmsModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_sms()
    }

    pub fn get_storage(
        &self,
    ) -> Result<Arc<crate::modules::storage::StorageModule>, ModuleRegistryError> {
        let registry = self.0.read().unwrap();
        registry.get_storage()
    }
}

impl fmt::Debug for ModuleRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0.read().unwrap();
        inner.fmt(f)
    }
}
