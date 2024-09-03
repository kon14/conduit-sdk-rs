use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::modules::Module;
use crate::prelude::ModuleRegistryError;
use crate::sd::ModuleName;

pub(crate) struct InnerModuleRegistry {
    connected_modules: HashMap<ModuleName, Module>,
    last_updated: DateTime<Utc>,
}

impl InnerModuleRegistry {
    pub fn add_module(
        &mut self,
        name: ModuleName,
        module: Module,
    ) -> Result<(), ModuleRegistryError> {
        if self.is_registered(&name) {
            return Err(ModuleRegistryError::AlreadyRegistered(name));
        }
        self.connected_modules.insert(name, module);
        self.last_updated = Utc::now();
        Ok(())
    }

    pub fn remove_module(&mut self, name: ModuleName) -> Result<Module, ModuleRegistryError> {
        if !self.is_registered(&name) {
            return Err(ModuleRegistryError::NotRegistered(name));
        }
        let module = self
            .connected_modules
            .remove(&name)
            .ok_or(ModuleRegistryError::NotRegistered(name));
        self.last_updated = Utc::now();
        module
    }

    pub fn get_module(&self, name: &ModuleName) -> Option<Module> {
        self.connected_modules.get(name).cloned()
    }

    pub fn is_registered(&self, name: &ModuleName) -> bool {
        self.connected_modules.contains_key(name)
    }

    pub fn get_authentication(
        &self,
    ) -> Result<Arc<crate::modules::authentication::AuthenticationModule>, ModuleRegistryError>
    {
        let module_name = ModuleName::Authentication;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Authentication(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_authorization(
        &self,
    ) -> Result<Arc<crate::modules::authorization::AuthorizationModule>, ModuleRegistryError> {
        let module_name = ModuleName::Authorization;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Authorization(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_chat(&self) -> Result<Arc<crate::modules::chat::ChatModule>, ModuleRegistryError> {
        let module_name = ModuleName::Chat;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Chat(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_database(
        &self,
    ) -> Result<Arc<crate::modules::database::DatabaseModule>, ModuleRegistryError> {
        let module_name = ModuleName::Database;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Database(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_email(
        &self,
    ) -> Result<Arc<crate::modules::email::EmailModule>, ModuleRegistryError> {
        let module_name = ModuleName::Email;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Email(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_forms(
        &self,
    ) -> Result<Arc<crate::modules::forms::FormsModule>, ModuleRegistryError> {
        let module_name = ModuleName::Forms;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Forms(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_push_notifications(
        &self,
    ) -> Result<Arc<crate::modules::push_notifications::PushNotificationsModule>, ModuleRegistryError>
    {
        let module_name = ModuleName::PushNotifications;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::PushNotifications(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_router(
        &self,
    ) -> Result<Arc<crate::modules::router::RouterModule>, ModuleRegistryError> {
        let module_name = ModuleName::Router;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Router(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_sms(&self) -> Result<Arc<crate::modules::sms::SmsModule>, ModuleRegistryError> {
        let module_name = ModuleName::SMS;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::SMS(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }

    pub fn get_storage(
        &self,
    ) -> Result<Arc<crate::modules::storage::StorageModule>, ModuleRegistryError> {
        let module_name = ModuleName::Storage;
        let module = self
            .get_module(&module_name)
            .ok_or(ModuleRegistryError::NotRegistered(module_name.clone()))?;
        match module {
            Module::Storage(m) => Ok(m),
            _ => Err(ModuleRegistryError::NotRegistered(module_name)),
        }
    }
}

impl Default for InnerModuleRegistry {
    fn default() -> Self {
        InnerModuleRegistry {
            connected_modules: Default::default(),
            last_updated: Utc::now(),
        }
    }
}

impl fmt::Debug for InnerModuleRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "ModuleRegistry {{\n")?;
            write!(f, "    connected_modules: [\n")?;
            for module in self.connected_modules.values().into_iter() {
                write!(f, "        {:?},\n", module)?;
            }
            write!(f, "    ],\n")?;
            write!(f, "    last_updated: {:?}\n", self.last_updated)?;
            write!(f, "}}")
        } else {
            let module = self.connected_modules.values().collect::<Vec<_>>(); // Collect keys for concise output
            write!(
                f,
                "ModuleRegistry {{ connected_modules: {:?}, last_updated: {:?} }}",
                module, self.last_updated
            )
        }
    }
}
