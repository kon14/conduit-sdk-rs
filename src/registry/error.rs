use thiserror::Error;

use crate::sd::ModuleName;

#[derive(Error, Debug)]
pub enum ModuleRegistryError {
    #[error("ModuleRegistryError: Already registered ({0})")]
    AlreadyRegistered(ModuleName),
    #[error("ModuleRegistryError: Not registered ({0})")]
    NotRegistered(ModuleName),
}
