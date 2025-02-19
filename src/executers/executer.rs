use crate::cli::Cli;
use crate::data::WsTaskData;
use crate::error::BError;
use crate::executers::{Docker, DockerImage, TaskExecuter};

use std::collections::HashMap;

pub struct CleanExecuter<'a> {
    task_data: &'a WsTaskData,
    cli: &'a Cli,
}

impl<'a> TaskExecuter for CleanExecuter<'a> {
    fn exec(
        &self,
        args_env_variables: &HashMap<String, String>,
        _dry_run: bool,
        interactive: bool,
    ) -> Result<(), BError> {
        self.cli.info(format!(
            "execute non-hlos clean task '{}'",
            self.task_data.name()
        ));
        let mut cmd_line: Vec<String> = vec![];
        let cmd: String = self.task_data.clean_cmd().to_owned();
        cmd_line.append(&mut vec![
            "cd".to_string(),
            self.task_data.build_dir().to_string_lossy().to_string(),
            "&&".to_string(),
        ]);
        let mut v: Vec<String> = cmd.split(' ').map(|s| s.to_string()).collect();
        cmd_line.append(&mut v);

        let mut docker_str: &str = "";
        if !self.task_data.docker_image().is_empty() && self.task_data.docker_image() != "NA" {
            docker_str = self.task_data.docker_image();
        }

        if !docker_str.is_empty() {
            let image: DockerImage = DockerImage::new(docker_str)?;
            let docker: Docker = Docker::new(image, interactive);
            docker.run_cmd(
                &mut cmd_line,
                args_env_variables,
                self.task_data.build_dir(),
                &self.cli,
            )?;
        } else {
            self.cli.check_call(&cmd_line, args_env_variables, true)?;
        }

        Ok(())
    }
}

impl<'a> CleanExecuter<'a> {
    pub fn new(cli: &'a Cli, task_data: &'a WsTaskData) -> Self {
        CleanExecuter { cli, task_data }
    }
}

pub struct BuildExecuter<'a> {
    cli: &'a Cli,
    task_data: &'a WsTaskData,
}

impl<'a> TaskExecuter for BuildExecuter<'a> {
    fn exec(
        &self,
        env_variables: &HashMap<String, String>,
        dry_run: bool,
        interactive: bool,
    ) -> Result<(), BError> {
        if dry_run {
            self.cli.info("Dry run. Skipping build!".to_string());
            return Ok(());
        }

        let exec_dir: &std::path::PathBuf = self.task_data.build_dir();
        let mut cmd_line: Vec<String> = vec![
            "cd".to_string(),
            exec_dir.to_string_lossy().to_string(),
            "&&".to_string(),
        ];
        let mut cmd: Vec<String> = self
            .task_data
            .build_cmd()
            .split(' ')
            .map(|c| c.to_string())
            .collect();
        cmd_line.append(&mut cmd);

        if !self.task_data.docker_image().is_empty() && self.task_data.docker_image() != "NA" {
            let image: DockerImage = DockerImage::new(self.task_data.docker_image())?;
            let docker: Docker = Docker::new(image, interactive);
            docker.run_cmd(&mut cmd_line, env_variables, exec_dir, &self.cli)?;
        } else {
            let env: HashMap<String, String> = env_variables.clone().into_iter().chain(self.cli.env().into_iter()).collect();
            self.cli.check_call(&cmd_line, &env, true)?;
        }
        Ok(())
    }
}

impl<'a> BuildExecuter<'a> {
    pub fn new(cli: &'a Cli, task_data: &'a WsTaskData) -> Self {
        BuildExecuter { cli, task_data }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tempdir::TempDir;

    use crate::cli::*;
    use crate::data::{WsBuildData, WsTaskData};
    use crate::executers::{BuildExecuter, CleanExecuter, TaskExecuter};
    use crate::helper::Helper;

    #[test]
    fn test_yaab_executer() {
        let temp_dir: TempDir =
            TempDir::new("yaab-test-dir").expect("Failed to create temp directory");
        let work_dir: PathBuf = temp_dir.into_path();
        let build_dir: PathBuf = work_dir.join("test-dir");
        let env_variables: HashMap<String, String> = HashMap::new();
        let json_build_config: &str = r#"
        {
            "version": "4",
            "name": "default",
            "description": "Test Description",
            "arch": "test-arch"
        }"#;
        let json_task_config: &str = r#"
        {
            "index": "1",
            "name": "task-name",
            "builddir": "test-dir",
            "build": "test.sh",
            "clean": "rm -rf test-dir"
        }"#;
        let data: WsBuildData = Helper::setup_build_data(&work_dir, Some(json_build_config), None);
        let task_data: WsTaskData =
            WsTaskData::from_str(json_task_config, &data).expect("Failed to parse task config");
        let mut mocked_system: MockSystem = MockSystem::new();
        mocked_system
            .expect_check_call()
            .with(mockall::predicate::eq(CallParams {
                cmd_line: vec![
                    "cd",
                    &build_dir.to_string_lossy().to_string(),
                    "&&",
                    "test.sh",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                env: HashMap::new(),
                shell: true,
            }))
            .once()
            .returning(|_x| Ok(()));
        mocked_system.expect_env().returning(|| HashMap::new());
        let cli: Cli = Cli::new(
            Box::new(BLogger::new()),
            Box::new(mocked_system),
            clap::Command::new("yaab"),
            Some(vec!["yaab"]),
        );
        let executer: BuildExecuter = BuildExecuter::new(&cli, &task_data);
        executer
            .exec(&env_variables, false, true)
            .expect("Failed to execute task");
    }

    #[test]
    fn test_yaab_executer_dry_run() {
        let temp_dir: TempDir =
            TempDir::new("yaab-test-dir").expect("Failed to create temp directory");
        let work_dir: PathBuf = temp_dir.into_path();
        let env_variables: HashMap<String, String> = HashMap::new();
        let json_build_config: &str = r#"
        {
            "version": "4",
            "name": "default",
            "description": "Test Description",
            "arch": "test-arch"
        }"#;
        let json_task_config: &str = r#"
        {
            "index": "1",
            "name": "task-name",
            "builddir": "test-dir",
            "build": "test.sh",
            "clean": "rm -rf test-dir"
        }"#;
        let data: WsBuildData = Helper::setup_build_data(&work_dir, Some(json_build_config), None);
        let task_data: WsTaskData =
            WsTaskData::from_str(json_task_config, &data).expect("Failed to parse task config");
        let mut mocked_logger: MockLogger = MockLogger::new();
        mocked_logger
            .expect_info()
            .with(mockall::predicate::eq(
                "Dry run. Skipping build!".to_string(),
            ))
            .once()
            .returning(|_x| ());
        let cli: Cli = Cli::new(
            Box::new(mocked_logger),
            Box::new(BSystem::new()),
            clap::Command::new("yaab"),
            Some(vec!["yaab"]),
        );
        let executer: BuildExecuter = BuildExecuter::new(&cli, &task_data);
        executer
            .exec(&env_variables, true, true)
            .expect("Failed to execute task");
    }

    /*
    #[test]
    fn test_bitbake_executer_docker() {
        let temp_dir: TempDir = TempDir::new("yaab-test-dir").expect("Failed to create temp directory");
        let work_dir: PathBuf = temp_dir.into_path();
        let build_dir: PathBuf = work_dir.join("test-dir");
        let env_variables: HashMap<String, String> = HashMap::new();
        let json_build_config: &str = r#"
        {
            "version": "4",
            "name": "default",
            "description": "Test Description",
            "arch": "test-arch"
        }"#;
        let json_task_config: &str = r#"
        {
            "index": "1",
            "name": "task-name",
            "type": "non-bitbake",
            "docker": "test-registry/task-docker:0.1",
            "builddir": "test-dir",
            "build": "test.sh",
            "clean": "rm -rf test-dir"
        }"#;
        let data: WsBuildData = Helper::setup_build_data(&work_dir, Some(json_build_config), None);
        let task_data: WsTaskData = WsTaskData::from_str(json_task_config, &data).expect("Failed to parse task config");
        let mut mocked_system: MockSystem = MockSystem::new();
        mocked_system
            .expect_check_call()
            .with(mockall::predicate::eq(CallParams {
                cmd_line: vec!["docker", "run", "test-registry/task-docker:0.1", "cd", &build_dir.to_string_lossy().to_string(), "&&", "test.sh"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                env: HashMap::new(),
                shell: true,
            }))
            .once()
            .returning(|_x| Ok(()));
        let cli: Cli = Cli::new(
            Box::new(BLogger::new()),
            Box::new(mocked_system),
            clap::Command::new("yaab"),
            Some(vec!["yaab"]),
        );
        let executer: NonBitbakeExecuter = NonBitbakeExecuter::new(&cli, &task_data);
        executer.exec(&env_variables, false, true).expect("Failed to execute task");
    }
    */

    #[test]
    fn test_yaab_clean_executer() {
        let temp_dir: TempDir =
            TempDir::new("yaab-test-dir").expect("Failed to create temp directory");
        let work_dir: PathBuf = temp_dir.into_path();
        let build_dir: PathBuf = work_dir.join("test-dir");
        let env_variables: HashMap<String, String> = HashMap::new();
        let json_build_config: &str = r#"
        {
            "version": "4",
            "name": "default",
            "description": "Test Description",
            "arch": "test-arch"
        }"#;
        let json_task_config: &str = r#"
        {
            "index": "1",
            "name": "task-name",
            "builddir": "test-dir",
            "build": "test.sh",
            "clean": "rm -rf dir-to-delete"
        }"#;
        let data: WsBuildData = Helper::setup_build_data(&work_dir, Some(json_build_config), None);
        let task_data: WsTaskData =
            WsTaskData::from_str(json_task_config, &data).expect("Failed to parse task config");
        let mut mocked_system: MockSystem = MockSystem::new();
        mocked_system
            .expect_check_call()
            .with(mockall::predicate::eq(CallParams {
                cmd_line: vec![
                    "cd",
                    &build_dir.to_string_lossy().to_string(),
                    "&&",
                    "rm",
                    "-rf",
                    "dir-to-delete",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                env: HashMap::new(),
                shell: true,
            }))
            .once()
            .returning(|_x| Ok(()));
        let cli: Cli = Cli::new(
            Box::new(BLogger::new()),
            Box::new(mocked_system),
            clap::Command::new("yaab"),
            Some(vec!["yaab"]),
        );
        let executer: CleanExecuter = CleanExecuter::new(&cli, &task_data);
        executer
            .exec(&env_variables, false, true)
            .expect("Failed to execute task");
    }
}
