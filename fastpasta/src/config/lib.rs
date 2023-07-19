//! Contains the [Config] super trait, and all the sub traits required by it
//!
//! Implementing the [Config] super trait is required by configs passed to structs in other modules as part of instantiation.

use super::{
    check::{CheckCommands, ChecksOpt, System},
    custom_checks::CustomChecksOpt,
    prelude::{InputOutputOpt, ViewOpt},
    util::UtilOpt,
};
use alice_protocol_reader::prelude::FilterOpt;

/// Super trait for all the traits that needed to be implemented by the config struct
// Generic traits that are required by the config struct
pub trait Config: Send + Sync + std::marker::Sized
where
    // Subtraits that group together related configuration options
    Self: UtilOpt + FilterOpt + InputOutputOpt + ChecksOpt + ViewOpt + CustomChecksOpt,
{
    /// Validate the arguments of the config
    fn validate_args(&self) -> Result<(), String> {
        if let Some(check) = self.check() {
            if let Some(target) = check.target() {
                if matches!(check, CheckCommands::Sanity { system } if matches!(system, Some(System::ITS_Stave)))
                {
                    return Err("Invalid config: Cannot check ITS stave with `check sanity`, instead use `check all its-stave`".to_string());
                }
                if !matches!(target, System::ITS_Stave) && self.check_its_trigger_period().is_some()
                {
                    return Err("Invalid config: Specifying trigger period has to be done with the `check all its-stave` command".to_string());
                }
            }
        }
        if self.any_errors_exit_code().is_some_and(|val| val == 0) {
            return Err("Invalid config: Exit code for any errors cannot be 0".to_string());
        }
        Ok(())
    }
}

impl<T> Config for &T
where
    T: Config,
{
    fn validate_args(&self) -> Result<(), String> {
        (*self).validate_args()
    }
}

impl<T> Config for Box<T>
where
    T: Config,
{
    fn validate_args(&self) -> Result<(), String> {
        (**self).validate_args()
    }
}
impl<T> Config for std::sync::Arc<T>
where
    T: Config,
{
    fn validate_args(&self) -> Result<(), String> {
        (**self).validate_args()
    }
}