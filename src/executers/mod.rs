pub mod customsubcmd;
pub mod docker;
pub mod executer;

pub use customsubcmd::CustomSubCmdExecuter;
pub use docker::Docker;
pub use docker::DockerImage;
pub use executer::{BuildExecuter, CleanExecuter};

use crate::error::BError;

use std::collections::HashMap;

pub trait TaskExecuter {
    fn exec(
        &self,
        _env_variables: &HashMap<String, String>,
        _dry_run: bool,
        _interactive: bool,
    ) -> Result<(), BError> {
        Ok(())
    }
}
