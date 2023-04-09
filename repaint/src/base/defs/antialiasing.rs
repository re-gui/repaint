

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The antialias mode.
pub enum AntialiasMode {
    /// No antialiasing.
    None,

    /// Exact antialiasing.
    Exact,

    /// Antialiasing using a `ny x nx` grid.
    SubPixel{nx: u16, ny: u16},
}