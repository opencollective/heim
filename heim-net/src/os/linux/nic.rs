use crate::Address;

/// Linux-specific extension for [Nic].
///
/// [Nic]: ../../struct.Nic.html
#[heim_derive::os_ext_for(crate::Nic, cfg(target_os = "linux"))]
pub trait NicExt {
    /// Returns broadcast address if available.
    fn broadcast(&self) -> Option<Address>;

    /// Returns `bool` indicating whether interface is broadcast.
    fn is_broadcast(&self) -> bool;

    /// Returns `bool` indicating whether interface is point-to-point.
    fn is_point_to_point(&self) -> bool;
}