//! Code associated with viewing a package from npm

use crate::package::PackageJson;
use semver::Version;
use serde::Deserialize;
use std::{
  borrow::Cow,
  collections::HashMap,
  path::Path,
};
use url_serde::SerdeUrl;

#[derive(Deserialize, Debug)]
pub struct ViewPkg<'p> {
  name: Cow<'p, str>,
  description: Cow<'p, str>,
  keywords: Vec<Cow<'p, str>>,
  version: Version,
  homepage: Cow<'p, str>,
  //bugs: crate::package::Bugs<'p>,
  license: Cow<'p, str>,
  main: Cow<'p, str>,
  //repository: crate::package::Repository<'p>,
  engines: HashMap<Cow<'p, str>, Cow<'p, str>>,
  dependencies: HashMap<Cow<'p, str>, Cow<'p, str>>,
  maintainers: HashMap<Cow<'p, str>, crate::package::Person<'p>>,
  dist: Dist<'p>,
}
#[derive(Deserialize, Debug)]
pub struct Dist<'p> {
  integrity: Cow<'p, str>,
  shasum: Cow<'p, str>,
  tarball: SerdeUrl,
  #[serde(rename = "fileCount")]
  file_count: usize,
  #[serde(rename = "unpackedSize")]
  unpacked_size: usize,
  #[serde(rename = "npm-signature")]
  npm_signature: Cow<'p, str>,
}
