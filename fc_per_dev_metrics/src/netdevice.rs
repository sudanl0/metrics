use crate::metrics::SharedIncMetric;
use serde::{Serialize, ser::SerializeMap};

/// Network-related metrics.
#[derive(Debug, Default, Serialize)]
pub struct NetPerDeviceMetrics {
    /// Number of times when activate failed on a network device.
    pub activate_fails: SharedIncMetric,
    /// Number of bytes received.
    pub rx_bytes_count: SharedIncMetric,
}
impl NetPerDeviceMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            activate_fails: SharedIncMetric::new(),
            rx_bytes_count: SharedIncMetric::new(),
        }
    }
}

// #[derive(Debug, Serialize)]
// #[serde(deny_unknown_fields)]
pub struct Net{
    pub(crate) id: String,
    pub(crate) metrics: NetPerDeviceMetrics,
}

impl Serialize for Net{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let dev = self.id.clone();
        let mut seq = serializer.serialize_map(Some(1))?;
        println!(">>{:?}", dev);
        seq.serialize_entry(&dev, &self.metrics)?;
        seq.end()
    }
}

impl Net{
    pub fn new(id: String) -> Net{
        Net{
            id,
            metrics: NetPerDeviceMetrics::new()
        }
    }
}