use std::env;
use termcolor::ColorChoice;

/// Colour mode for output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColourMode {
    /// Auto-detect based on TTY
    Auto,
    /// Always use colours
    Always,
    /// Never use colours
    Never,
}

impl ColourMode {
    /// Determine colour mode from CLI flags and environment variables
    ///
    /// Priority:
    /// 1. CLI flags (--colour or --no-colour)
    /// 2. NO_COLOR environment variable
    /// 3. CLICOLOR_FORCE environment variable
    /// 4. CLICOLOR environment variable
    /// 5. Auto-detect TTY (default)
    pub fn from_env(colour_flag: bool, no_colour_flag: bool) -> Self {
        // CLI flags have highest priority
        if colour_flag {
            return ColourMode::Always;
        }
        if no_colour_flag {
            return ColourMode::Never;
        }

        // Check NO_COLOR (universal opt-out)
        if env::var("NO_COLOR").is_ok() {
            return ColourMode::Never;
        }

        // Check CLICOLOR_FORCE (force colours on)
        if let Ok(val) = env::var("CLICOLOR_FORCE")
            && val != "0"
        {
            return ColourMode::Always;
        }

        // Check CLICOLOR (enable/disable with TTY respect)
        if let Ok(val) = env::var("CLICOLOR") {
            if val == "0" {
                return ColourMode::Never;
            }
            // Non-zero CLICOLOR means "use colours if TTY", which is Auto
            return ColourMode::Auto;
        }

        // Default: auto-detect TTY
        ColourMode::Auto
    }

    /// Convert to termcolor's ColorChoice
    pub(crate) fn to_color_choice(self) -> ColorChoice {
        match self {
            ColourMode::Auto => ColorChoice::Auto,
            ColourMode::Always => ColorChoice::Always,
            ColourMode::Never => ColorChoice::Never,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colour_flag_takes_priority() {
        // CLI flags should override environment variables
        assert_eq!(ColourMode::from_env(true, false), ColourMode::Always);
        assert_eq!(ColourMode::from_env(false, true), ColourMode::Never);
    }

    #[test]
    fn test_no_color_env() {
        // Clean environment first
        unsafe {
            env::remove_var("NO_COLOR");
            env::remove_var("CLICOLOR_FORCE");
            env::remove_var("CLICOLOR");
        }

        // Set NO_COLOR environment variable
        unsafe {
            env::set_var("NO_COLOR", "1");
        }
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Never);
        unsafe {
            env::remove_var("NO_COLOR");
        }
    }

    #[test]
    fn test_clicolor_force_env() {
        // Clean environment first
        unsafe {
            env::remove_var("NO_COLOR");
            env::remove_var("CLICOLOR_FORCE");
            env::remove_var("CLICOLOR");
        }

        // CLICOLOR_FORCE=1 should force colours on
        unsafe {
            env::set_var("CLICOLOR_FORCE", "1");
        }
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Always);
        unsafe {
            env::remove_var("CLICOLOR_FORCE");
        }

        // CLICOLOR_FORCE=0 should not force colours
        unsafe {
            env::set_var("CLICOLOR_FORCE", "0");
        }
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Auto);
        unsafe {
            env::remove_var("CLICOLOR_FORCE");
        }
    }

    #[test]
    fn test_clicolor_env() {
        // Clean environment first
        unsafe {
            env::remove_var("NO_COLOR");
            env::remove_var("CLICOLOR_FORCE");
            env::remove_var("CLICOLOR");
        }

        // CLICOLOR=0 should disable colours
        unsafe {
            env::set_var("CLICOLOR", "0");
        }
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Never);
        unsafe {
            env::remove_var("CLICOLOR");
        }

        // CLICOLOR=1 should use auto-detection
        unsafe {
            env::set_var("CLICOLOR", "1");
        }
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Auto);
        unsafe {
            env::remove_var("CLICOLOR");
        }
    }

    #[test]
    fn test_env_priority() {
        // CLI flags > NO_COLOR > CLICOLOR_FORCE > CLICOLOR
        unsafe {
            env::set_var("NO_COLOR", "1");
            env::set_var("CLICOLOR_FORCE", "1");
            env::set_var("CLICOLOR", "1");
        }

        // CLI flag should win
        assert_eq!(ColourMode::from_env(true, false), ColourMode::Always);

        // NO_COLOR should win over CLICOLOR_FORCE
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Never);

        unsafe {
            env::remove_var("NO_COLOR");
        }
        // CLICOLOR_FORCE should win over CLICOLOR
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Always);

        unsafe {
            env::remove_var("CLICOLOR_FORCE");
            env::remove_var("CLICOLOR");
        }
    }

    #[test]
    fn test_default_auto() {
        // With no flags or env vars, should default to Auto
        unsafe {
            env::remove_var("NO_COLOR");
            env::remove_var("CLICOLOR_FORCE");
            env::remove_var("CLICOLOR");
        }
        assert_eq!(ColourMode::from_env(false, false), ColourMode::Auto);
    }
}
