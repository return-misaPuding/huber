use huber_common::model::release::{Release, ReleaseType, ReleaseTargetType, ReleaseManagement};

pub fn release() -> Release {
    Release {
        name: "velero".to_string(),
        version: "latest".to_string(),
        type_: ReleaseType::Github {
            owner: "vmware-tanzu".to_string(),
            repo: "velero".to_string(),
        },
        detail: None,
        targets: Some(vec![
            ReleaseTargetType::LinuxAmd64(ReleaseManagement {
                artifact_templates: Some(vec!("velero-{version}-{os}-{arch}.tar.gz".to_string())), // velero-v1.4.3-linux-amd64.tar.gz
                install_commands: None, // untar, walk through all executables, then install
                uninstall_commands: None,
                upgrade_commands: None,
            })
        ]),
    }
}