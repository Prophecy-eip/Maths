/// Enum containing the two ways to apply a Modifier
///
/// # Attributes
///
/// AlwaysTrue (bool): When the Modifier always apply
///
/// Function (fn(crate::regiment::Regiment, crate::regiment::Regiment) -> bool): When the Modifier need a function to specify if it can be applied in this specific case
pub enum ModifierCheck {
    AlwaysTrue(bool),
    Function(fn(crate::regiment::Regiment, crate::regiment::Regiment) -> bool),
}

/// Function to be REMOVED it's only an example
/// The "_" is only here to remove the warning of unused parameter
/// Add #[must_use] to ensure the return is used
#[must_use]
pub fn check_mockfunction(
    _attacking: crate::regiment::Regiment,
    _defending: crate::regiment::Regiment,
) -> bool {
    true
}
