use std::path::Path;

use super::PackageFile;
use crate::{NoArchType, Version};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr, OneOrMany};

/// A representation of the `index.json` file found in package archives.
///
/// The `index.json` file contains information about the package build and dependencies of the package.
/// This data makes up the repodata.json file in the repository.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct IndexJson {
    /// The lowercase name of the package
    pub name: String,

    /// The version of the package
    #[serde_as(as = "DisplayFromStr")]
    pub version: Version,

    /// The build string of the package.
    pub build: String,

    /// The build number of the package. This is also included in the build string.
    pub build_number: u64,

    /// Optionally, the architecture the package is build for.
    pub arch: Option<String>,

    /// If this package is independent of architecture this field specifies in what way. See
    /// [`NoArchType`] for more information.
    #[serde(skip_serializing_if = "NoArchType::is_none")]
    pub noarch: NoArchType,

    /// Optionally, the OS the package is build for.
    pub platform: Option<String>,

    /// Optionally, the license
    pub license: Option<String>,

    /// Optionally, the license family
    pub license_family: Option<String>,

    /// The dependencies of the package
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends: Vec<String>,

    /// The package constraints of the package
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constrains: Vec<String>,

    /// Track features are nowadays only used to downweight packages (ie. give them less priority). To
    /// that effect, the number of track features is counted (number of commas) and the package is downweighted
    /// by the number of track_features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde_as(as = "OneOrMany<_>")]
    pub track_features: Vec<String>,

    /// Features are a deprecated way to specify different feature sets for the conda solver. This is not
    /// supported anymore and should not be used. Instead, `mutex` packages should be used to specify
    /// mutually exclusive features.
    pub features: Option<String>,

    /// The timestamp when this package was created
    pub timestamp: Option<u64>,

    /// The subdirectory that contains this package
    pub subdir: Option<String>,
}

impl PackageFile for IndexJson {
    fn package_path() -> &'static Path {
        Path::new("info/index.json")
    }

    fn from_str(str: &str) -> Result<Self, std::io::Error> {
        serde_json::from_str(str).map_err(Into::into)
    }
}

#[cfg(test)]
mod test {
    use super::{IndexJson, PackageFile};

    #[test]
    pub fn test_reconstruct_index_json() {
        let package_dir = tempfile::tempdir().unwrap();
        rattler_package_streaming::fs::extract(
            &crate::get_test_data_dir().join("zlib-1.2.8-vc10_0.tar.bz2"),
            package_dir.path(),
        )
        .unwrap();

        insta::assert_yaml_snapshot!(IndexJson::from_package_directory(package_dir.path()).unwrap());
    }

    #[test]
    #[cfg(unix)]
    pub fn test_reconstruct_index_json_with_symlinks() {
        let package_dir = tempfile::tempdir().unwrap();
        rattler_package_streaming::fs::extract(
            &crate::get_test_data_dir().join("with-symlinks/zlib-1.2.8-3.tar.bz2"),
            package_dir.path(),
        )
        .unwrap();

        let package_dir = package_dir.into_path();
        println!("{}", package_dir.display());

        insta::assert_yaml_snapshot!(IndexJson::from_package_directory(&package_dir).unwrap());
    }
}
