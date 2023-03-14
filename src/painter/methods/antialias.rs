use std::error::Error;

use strum::Display;

use crate::base::defs::antialiasing::AntialiasMode;



/// Methods related to antialiasing.
pub trait AntialiasMethods {
    /// Returns whether this painter supports antialiasing.
    fn has_antialias(&self) -> bool {
        false
    }

    /// Returns whether this painter can set the given antialias mode.
    fn can_set_antialias_mode(&self, _mode: AntialiasMode) -> bool {
        self.has_antialias()
    }

    /// Returns whether this painter can change the antialias mode at this time.
    /// Some painters may not be able to change the antialias mode while drawing but only
    /// before or after. ???
    fn can_change_antialias_mode_now(&self) -> bool {
        false
    }

    /// Returns the current antialias mode.
    fn antialias_mode(&self) -> AntialiasMode {
        AntialiasMode::None
    }

    /// Sets the antialias mode. If the given antialias mode is not supported by this painter,
    /// the effective antialias mode will be undefined.
    fn set_antialias_mode(&mut self, _mode: AntialiasMode) -> Result<(), AntialiasSetError> {
        Err(AntialiasSetError::AntialiasNotSupported)
    }

    /// Tells whether antialiasing is enabled.
    fn antialiased(&self) -> bool {
        self.antialias_mode() != AntialiasMode::None
    }
}

/// Error that could occur when setting the antialias mode.
#[derive(Debug, Display)]
pub enum AntialiasSetError {
    /// The selected antialias mode is not supported by this painter.
    AntialiasNotSupported,

    /// The selected antialias mode is invalid, this could be returned when invalid
    /// values are passed to the [`set_antialias_mode`] method.
    /// 
    /// [`set_antialias_mode`]: AntialiasMethods::set_antialias_mode
    InvalidMode,

    /// The antialias mode cannot be changed at this time.
    CannotChangeNow,

    /// An unknown error occurred.
    Other(Box<dyn Error>)
}

impl Error for AntialiasSetError {}