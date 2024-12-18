pub mod customsubcmd;
pub mod docker;
pub mod hlos;
pub mod nonhlos;

pub use customsubcmd::CustomSubCmdExecuter;
pub use docker::Docker;
pub use docker::DockerImage;
pub use hlos::{HLOSBuildExecuter, HLOSCleanExecuter};
pub use nonhlos::{NonHLOSBuildExecuter, NonHLOSCleanExecuter};

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
