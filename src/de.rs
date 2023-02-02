
use kclvm_runner::{exec_program, ExecProgramArgs};
use serde::de::DeserializeOwned;

pub fn to_instance<T: DeserializeOwned>(filenames: Vec<String>) -> anyhow::Result<T> {
    let args = ExecProgramArgs {
        k_filename_list: filenames,
        ..ExecProgramArgs::default()
    };

    exec_program(&args, 1)
        .map(|result| {
            let content = result.yaml_result;
            let yaml: T = serde_yaml::from_str(&content).unwrap();
            yaml
        })
        .map_err(|err| anyhow::anyhow!(err))
}
