//! [CountersOpt] trait for getting expected counter values to compare against the data.`
//! [Counters] struct generated by deserializing JSON with expected counter values.
use serde_derive::{Deserialize, Serialize};

/// Trait for the configuration of various expected counters in the data.
pub trait CountersOpt {
    /// Get the number of CDPS expected in the data, if it is set.
    fn cdps(&self) -> Option<u32>;

    /// Get the number of sent Triggers expected in the data, if it is set.
    fn triggers_sent(&self) -> Option<u32>;
}

impl<T> CountersOpt for &T
where
    T: CountersOpt,
{
    fn cdps(&self) -> Option<u32> {
        (*self).cdps()
    }

    fn triggers_sent(&self) -> Option<u32> {
        (*self).triggers_sent()
    }
}

impl<T> CountersOpt for Box<T>
where
    T: CountersOpt,
{
    fn cdps(&self) -> Option<u32> {
        (**self).cdps()
    }

    fn triggers_sent(&self) -> Option<u32> {
        (**self).triggers_sent()
    }
}

impl<T> CountersOpt for std::sync::Arc<T>
where
    T: CountersOpt,
{
    fn cdps(&self) -> Option<u32> {
        (**self).cdps()
    }

    fn triggers_sent(&self) -> Option<u32> {
        (**self).triggers_sent()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Counters {
    cdps: Option<u32>,

    triggers_sent: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let counters = Counters {
            cdps: None,
            triggers_sent: None,
        };
        let json = serde_json::to_string_pretty(&counters).unwrap();
        println!("{json}");
        assert_eq!(
            json,
            r#"{
  "cdps": null,
  "triggers_sent": null
}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{ "cdps": 1, "triggers_sent": 1 }"#;
        let counters: Counters = serde_json::from_str(json).unwrap();
        assert_eq!(counters.cdps, Some(1));

        let json = r#"{ "cdps": null, "triggers_sent": null }"#;
        let counters: Counters = serde_json::from_str(json).unwrap();
        assert_eq!(counters.cdps, None);
    }
}
