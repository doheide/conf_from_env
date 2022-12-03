pub use cfe_progmacro::SetStructFromEnv;

use std::fmt;
use std::fmt::Debug;

pub trait SetStructFromEnv {
    fn set_from_env() -> Self;
}


#[derive(Clone)]
pub struct ConfigAndSecrets<C: SetStructFromEnv + Debug + Clone, S: SetStructFromEnv + Clone> {
    pub conf: C,
    pub secrets: S
}
impl<C: SetStructFromEnv + Debug + Clone, S: SetStructFromEnv + Clone> ConfigAndSecrets<C, S> {
    pub fn from_env() -> Self {
        ConfigAndSecrets{
            conf: C::set_from_env(), secrets: S::set_from_env()
        }
    }
}
impl<C: SetStructFromEnv + Debug + Clone, S: SetStructFromEnv + Clone> Debug for ConfigAndSecrets<C, S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ConfigAndSecrets").field("conf", &self.conf)
            .finish_non_exhaustive()?;
        fmt.write_fmt(format_args!(" (Secrets not shown)"))
    }
}

// fmt.write_fmt(format_args!("{:?}", self.conf))?;
// fmt.write("Secrets not shown")?;


