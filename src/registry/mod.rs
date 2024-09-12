mod error;
mod inner;

pub use error::ModuleRegistryError;
use inner::InnerModuleRegistry;

use crate::error::ConduitSdkError;
use crate::modules::Module;
use crate::sd::{ModuleName, ServiceDiscoveryState};
use std::fmt;
use std::sync::Arc;
use tokio::sync::{watch, RwLock};
use tonic::transport::Channel;

/// A locally tracked registry of connected modules.
#[derive(Default, Clone)]
pub struct ModuleRegistry(Arc<RwLock<InnerModuleRegistry>>);

impl ModuleRegistry {
    /// Instantiates a new `ModuleRegistry`, background-synchronizing it based on a channel of `ServiceDiscoveryState` updates.<br />
    /// Explicitly tracked modules are auto-instantiated and registered as soon as they become available.
    pub fn sync_from_channel(
        mut rx: watch::Receiver<ServiceDiscoveryState>,
        tracked_modules: Vec<ModuleName>,
    ) -> Self {
        let registry = Self::default();
        let task_registry = registry.clone();
        tokio::spawn(async move {
            while rx.changed().await.is_ok() {
                let state = rx.borrow().clone();
                for module_name in tracked_modules.clone().into_iter() {
                    if let Some(state) = state.module_states.get(&module_name) {
                        let res = task_registry.add_module_if_not_exists(
                            &module_name,
                            || async {
                                let grpc_address = state.grpc_address.clone();
                                let channel = Channel::from_shared(grpc_address.clone())
                                    .unwrap()
                                    .connect()
                                    .await
                                    .map_err(|err|
                                        ConduitSdkError::ConnectionFailure(
                                            format!("Failed to establish a connection to {module_name}@{grpc_address}!"),
                                            Some(Box::new(err)),
                                        )
                                    )?;
                                Ok(Module::new(module_name.clone(), grpc_address, channel))
                            },
                        ).await;
                        if let Err(err) = res {
                            log::error!("{}", err);
                        };
                    }
                    // TODO - Figure out how to optimally handle:
                    // - removed modules
                    // - replaced module instances
                }
            }
        });
        registry
    }

    pub async fn add_module(
        &self,
        name: ModuleName,
        module: Module,
    ) -> Result<(), ModuleRegistryError> {
        let mut registry = self.0.write().await;
        registry.add_module(name, module)
    }

    /// Adds a new module to the registry if not already registered.<br />
    /// Accepts a closure for on-demand instantiation of the module struct.<br />
    /// Returns `true` for new entry additions and `false` for ignored existing entries.
    pub async fn add_module_if_not_exists<F, Fut>(
        &self,
        name: &ModuleName,
        get_module: F,
    ) -> Result<bool, ConduitSdkError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Module, ConduitSdkError>>,
    {
        let mut registry = self.0.write().await;
        registry.add_module_if_not_exists(name, get_module).await
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
