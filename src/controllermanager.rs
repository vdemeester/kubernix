use crate::{
    config::Config, kubeconfig::KubeConfig, pki::Pki, process::Process,
};
use failure::Fallible;
use log::info;

pub struct ControllerManager {
    process: Process,
}

impl ControllerManager {
    pub fn new(
        config: &Config,
        pki: &Pki,
        kubeconfig: &KubeConfig,
    ) -> Fallible<ControllerManager> {
        info!("Starting Controller Manager");

        let mut process = Process::new(
            config,
            &[
                "kube-controller-manager".to_owned(),
                "--bind-address=0.0.0.0".to_owned(),
                "--cluster-cidr=10.200.0.0/16".to_owned(),
                "--cluster-name=kubernetes".to_owned(),
                format!(
                    "--cluster-signing-cert-file={}",
                    pki.ca_cert.display()
                ),
                format!("--cluster-signing-key-file={}", pki.ca_key.display()),
                format!(
                    "--kubeconfig={}",
                    kubeconfig.controller_manager.display()
                ),
                "--leader-elect=true".to_owned(),
                format!("--root-ca-file={}", pki.ca_cert.display()),
                format!(
                    "--service-account-private-key-file={}",
                    pki.service_account_key.display()
                ),
                "--service-cluster-ip-range=10.32.0.0/24".to_owned(),
                "--use-service-account-credentials=true".to_owned(),
                "--v=2".to_owned(),
            ],
        )?;

        process.wait_ready("Serving securely")?;
        info!("Controller Manager is ready");
        Ok(ControllerManager { process })
    }

    pub fn stop(&mut self) -> Fallible<()> {
        self.process.stop()?;
        Ok(())
    }
}
