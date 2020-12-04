use anyhow::Result;
use pasture_core::containers::PointBuffer;

/// Base trait for all types that support writing point data
pub trait PointWriter {
    /// Write the points in the given `PointBuffer` to the associated `PointWriter`.
    fn write(&mut self, points: &dyn PointBuffer) -> Result<()>;
}
