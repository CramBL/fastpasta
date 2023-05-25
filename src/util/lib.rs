//! Contains the [Config] super trait, and all the sub traits required by it
//!
//! Implementing the [Config] super trait is required by configs passed to structs in other modules as part of instantiation.
use std::{fmt::Display, sync::Arc};

use super::config::{
    filter::FilterOpt,
    view::{View, ViewOpt},
    Check,
};

/// Super trait for all the traits that needed to be implemented by the config struct
// Generic traits that are required by the config struct
pub trait Config: Send + Sync + std::marker::Sized
where
    // Subtraits that group together related configuration options
    Self: UtilOpt + FilterOpt + InputOutputOpt + ChecksOpt + ViewOpt,
{
    /// Validate the arguments of the config
    fn validate_args(&self) -> Result<(), String> {
        if let Some(check) = self.check() {
            if let Some(target) = check.target() {
                if matches!(target, super::config::System::ITS_Stave) {
                    if self.filter_its_stave().is_none() {
                        return Err("Cannot check ITS stave without specifying a stave".to_string());
                    }
                } else if self.check_its_trigger_period().is_some() {
                    return Err("Specifying trigger period has to be done with the `check all its_stave` command".to_string());
                }
            }
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
impl<T> Config for Arc<T>
where
    T: Config,
{
    fn validate_args(&self) -> Result<(), String> {
        (**self).validate_args()
    }
}

/// Trait for all small utility options that are not specific to any other trait
pub trait UtilOpt {
    /// Verbosity level of the logger: 0 = error, 1 = warn, 2 = info, 3 = debug, 4 = trace
    fn verbosity(&self) -> u8;
    /// Maximum number of errors to tolerate before exiting
    fn max_tolerate_errors(&self) -> u32;
}

impl<T> UtilOpt for &T
where
    T: UtilOpt,
{
    fn verbosity(&self) -> u8 {
        (*self).verbosity()
    }
    fn max_tolerate_errors(&self) -> u32 {
        (*self).max_tolerate_errors()
    }
}

impl<T> UtilOpt for &mut T
where
    T: UtilOpt,
{
    fn verbosity(&self) -> u8 {
        (**self).verbosity()
    }
    fn max_tolerate_errors(&self) -> u32 {
        (**self).max_tolerate_errors()
    }
}

impl<T> UtilOpt for Box<T>
where
    T: UtilOpt,
{
    fn verbosity(&self) -> u8 {
        (**self).verbosity()
    }

    fn max_tolerate_errors(&self) -> u32 {
        (**self).max_tolerate_errors()
    }
}

impl<T> UtilOpt for Arc<T>
where
    T: UtilOpt,
{
    fn verbosity(&self) -> u8 {
        (**self).verbosity()
    }

    fn max_tolerate_errors(&self) -> u32 {
        (**self).max_tolerate_errors()
    }
}

/// Trait for all input/output options
pub trait InputOutputOpt {
    /// Input file to read from.
    fn input_file(&self) -> &Option<std::path::PathBuf>;
    /// Determine from args if payload should be skipped at input
    fn skip_payload(&self) -> bool;
    /// Output file to write to.
    fn output(&self) -> &Option<std::path::PathBuf>;
    /// Output mode of the data writing (file, stdout, none)
    fn output_mode(&self) -> DataOutputMode;
}

impl<T> InputOutputOpt for &T
where
    T: InputOutputOpt,
{
    fn input_file(&self) -> &Option<std::path::PathBuf> {
        (*self).input_file()
    }
    fn skip_payload(&self) -> bool {
        (*self).skip_payload()
    }
    fn output(&self) -> &Option<std::path::PathBuf> {
        (*self).output()
    }
    fn output_mode(&self) -> DataOutputMode {
        (*self).output_mode()
    }
}

impl<T> InputOutputOpt for Box<T>
where
    T: InputOutputOpt,
{
    fn input_file(&self) -> &Option<std::path::PathBuf> {
        (**self).input_file()
    }
    fn skip_payload(&self) -> bool {
        (**self).skip_payload()
    }
    fn output(&self) -> &Option<std::path::PathBuf> {
        (**self).output()
    }
    fn output_mode(&self) -> DataOutputMode {
        (**self).output_mode()
    }
}
impl<T> InputOutputOpt for Arc<T>
where
    T: InputOutputOpt,
{
    fn input_file(&self) -> &Option<std::path::PathBuf> {
        (**self).input_file()
    }
    fn skip_payload(&self) -> bool {
        (**self).skip_payload()
    }
    fn output(&self) -> &Option<std::path::PathBuf> {
        (**self).output()
    }
    fn output_mode(&self) -> DataOutputMode {
        (**self).output_mode()
    }
}

/// Trait for all check options.
pub trait ChecksOpt {
    /// Type of Check to perform.
    fn check(&self) -> Option<Check>;

    /// Return the check on ITS trigger period if it is set.
    fn check_its_trigger_period(&self) -> Option<u16>;
}

impl<T> ChecksOpt for &T
where
    T: ChecksOpt,
{
    fn check(&self) -> Option<Check> {
        (*self).check()
    }
    fn check_its_trigger_period(&self) -> Option<u16> {
        (*self).check_its_trigger_period()
    }
}

impl<T> ChecksOpt for Box<T>
where
    T: ChecksOpt,
{
    fn check(&self) -> Option<Check> {
        (**self).check()
    }
    fn check_its_trigger_period(&self) -> Option<u16> {
        (**self).check_its_trigger_period()
    }
}
impl<T> ChecksOpt for Arc<T>
where
    T: ChecksOpt,
{
    fn check(&self) -> Option<Check> {
        (**self).check()
    }
    fn check_its_trigger_period(&self) -> Option<u16> {
        (**self).check_its_trigger_period()
    }
}

/// Enum for all possible data output modes.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DataOutputMode {
    /// Write to a file.
    File,
    /// Write to stdout.
    Stdout,
    /// Do not write data out.
    None,
}

impl Display for DataOutputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataOutputMode::File => write!(f, "File"),
            DataOutputMode::Stdout => write!(f, "Stdout"),
            DataOutputMode::None => write!(f, "None"),
        }
    }
}

#[allow(missing_docs)]
pub mod test_util {
    use super::*;
    use crate::util::config::filter::FilterOpt;
    #[derive(Debug, Clone)]

    /// Complete configurable Mock config for testing
    pub struct MockConfig {
        pub check: Option<Check>,
        pub view: Option<View>,
        pub filter_link: Option<u8>,
        pub filter_fee: Option<u16>,
        pub filter_its_stave: Option<String>,
        pub verbosity: u8,
        pub max_tolerate_errors: u32,
        pub input_file: Option<std::path::PathBuf>,
        pub skip_payload: bool,
        pub output: Option<std::path::PathBuf>,
        pub output_mode: DataOutputMode,
        pub its_trigger_period: Option<u16>,
    }

    impl Default for MockConfig {
        fn default() -> Self {
            Self {
                check: None,
                view: None,
                filter_link: None,
                filter_fee: None,
                filter_its_stave: None,
                verbosity: 0,
                max_tolerate_errors: 0,
                input_file: None,
                skip_payload: false,
                output: None,
                output_mode: DataOutputMode::None,
                its_trigger_period: None,
            }
        }
    }

    impl Config for MockConfig {}
    impl ChecksOpt for MockConfig {
        fn check(&self) -> Option<Check> {
            self.check.clone()
        }
        fn check_its_trigger_period(&self) -> Option<u16> {
            self.its_trigger_period
        }
    }
    impl ViewOpt for MockConfig {
        fn view(&self) -> Option<View> {
            self.view.clone()
        }
    }
    impl FilterOpt for MockConfig {
        fn filter_link(&self) -> Option<u8> {
            self.filter_link
        }

        fn filter_fee(&self) -> Option<u16> {
            self.filter_fee
        }

        fn filter_its_stave(&self) -> Option<u16> {
            if let Some(stave_layer) = &self.filter_its_stave {
                // Start with something like "l2_1"
                // 1. check if the first char is an L, if so, it's the Lx_x format
                if stave_layer.to_uppercase().starts_with('L') {
                    Some(
                        crate::words::its::layer_stave_string_to_feeid(stave_layer)
                            .expect("Invalid FEE ID"),
                    )
                } else {
                    panic!("Invalid ITS layer & stave format, expected L[layer numer]_[stave number], e.g. L2_1, got {stave_layer}")
                }
            } else {
                None
            }
        }
    }
    impl UtilOpt for MockConfig {
        fn verbosity(&self) -> u8 {
            self.verbosity
        }

        fn max_tolerate_errors(&self) -> u32 {
            self.max_tolerate_errors
        }
    }
    impl InputOutputOpt for MockConfig {
        fn input_file(&self) -> &Option<std::path::PathBuf> {
            &self.input_file
        }

        fn skip_payload(&self) -> bool {
            self.skip_payload
        }

        fn output(&self) -> &Option<std::path::PathBuf> {
            &self.output
        }

        fn output_mode(&self) -> DataOutputMode {
            self.output_mode
        }
    }
}
