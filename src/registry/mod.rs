mod error;
mod inner;

pub use error::ModuleRegistryError;
use inner::InnerModuleRegistry;

use crate::modules::Module;
use crate::sd::ModuleName;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A locally tracked registry of connected modules.
#[derive(Default)]
pub struct ModuleRegistry(Arc<RwLock<InnerModuleRegistry>>);

impl ModuleRegistry {
    pub async fn add_module(
        &self,
        name: ModuleName,
        module: Module,
    ) -> Result<(), ModuleRegistryError> {
        let mut registry = self.0.write().await;
        registry.add_module(name, module)
    }

    pub async fn remove_module(&self, name: ModuleName) -> Result<Module, ModuleRegistryError> {
        let mut registry = self.0.write().await;
        registry.remove_module(name)
    }

    pub async fn get_module(&self, name: &ModuleName) -> Option<Module> {
        let registry = self.0.read().await;
        registry.get_module(name)
    }

    pub async fn is_registered(&self, name: &ModuleName) -> bool {
        let registry = self.0.read().await;
        registry.is_registered(name)
    }

    pub async fn get_authentication(
        &self,
    ) -> Result<Arc<crate::modules::authentication::AuthenticationModule>, ModuleRegistryError>
    {
        let registry = self.0.read().await;
        registry.get_authentication()
    }

    pub async fn get_authorization(
        &self,
    ) -> Result<Arc<crate::modules::authorization::AuthorizationModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_authorization()
    }

    pub async fn get_chat(
        &self,
    ) -> Result<Arc<crate::modules::chat::ChatModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_chat()
    }

    pub async fn get_database(
        &self,
    ) -> Result<Arc<crate::modules::database::DatabaseModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_database()
    }

    pub async fn get_email(
        &self,
    ) -> Result<Arc<crate::modules::email::EmailModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_email()
    }

    pub async fn get_forms(
        &self,
    ) -> Result<Arc<crate::modules::forms::FormsModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_forms()
    }

    pub async fn get_push_notifications(
        &self,
    ) -> Result<Arc<crate::modules::push_notifications::PushNotificationsModule>, ModuleRegistryError>
    {
        let registry = self.0.read().await;
        registry.get_push_notifications()
    }

    pub async fn get_router(
        &self,
    ) -> Result<Arc<crate::modules::router::RouterModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_router()
    }

    pub async fn get_sms(
        &self,
    ) -> Result<Arc<crate::modules::sms::SmsModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_sms()
    }

    pub async fn get_storage(
        &self,
    ) -> Result<Arc<crate::modules::storage::StorageModule>, ModuleRegistryError> {
        let registry = self.0.read().await;
        registry.get_storage()
    }
}

impl fmt::Debug for ModuleRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.try_read() {
            Ok(inner) => inner.fmt(f),
            Err(_) => write!(f, "ModuleRegistry [Locked]"),
        }
    }
}
