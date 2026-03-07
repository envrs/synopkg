use {
  crate::{
    cli::Cli,
    rcfile::{error::RcfileError, Rcfile},
  },
  log::debug,
  serde_json::Value,
  std::fs,
};

pub fn try_from_package_json_config_property(cli: &Cli) -> Option<Result<Rcfile, RcfileError>> {
  let package_json_path = cli.cwd.join("package.json");
  package_json_path
    .exists()
    .then_some(&package_json_path)
    .and_then(|path| fs::read_to_string(path).ok())
    .and_then(|contents| serde_json::from_str::<Value>(&contents).ok())
    .and_then(|mut package_json| {
      package_json
        .get_mut("synopkg")
        .inspect(|_| debug!("Found .synopkg property in package.json"))
        .map(|config| config.take())
        .or_else(|| {
          package_json
            .pointer_mut("/config/synopkg")
            .inspect(|_| debug!("Found .config.synopkg property in package.json"))
            .map(|config| config.take())
        })
        .map(|synopkg_config| serde_json::from_value(synopkg_config).map_err(RcfileError::PackageJsonConfigInvalid))
    })
}
