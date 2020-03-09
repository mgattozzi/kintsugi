//! Code associated with dealing with the monstrosity that is `package.json`

use semver::Version;
use serde::Deserialize;
use std::{
  borrow::Cow,
  collections::HashMap,
  path::Path,
};
use url_serde::SerdeUrl;

#[derive(Deserialize, Debug)]
pub struct PackageJson<'p> {
  pub name: Option<Cow<'p, str>>,
  pub version: Option<Cow<'p, str>>,
  pub description: Option<Cow<'p, str>>,
  #[serde(default)]
  pub keywords: Vec<Cow<'p, str>>,
  pub homepage: Option<Cow<'p, str>>,
  pub bugs: Option<Bugs<'p>>,
  pub license: Option<Cow<'p, str>>,
  pub author: Option<PersonFormat<'p>>,
  #[serde(default)]
  pub contributors: Vec<Cow<'p, str>>,
  #[serde(default)]
  pub files: Vec<Cow<'p, str>>,
  pub main: Option<Cow<'p, str>>,
  pub browser: Option<Cow<'p, str>>,
  pub bin: Option<Bin<'p>>,
  pub man: Option<Man<'p>>,
  pub directories: Option<Directories<'p>>,
  pub repository: Option<Repository<'p>>,
  #[serde(default)]
  pub scripts: HashMap<Cow<'p, str>, Cow<'p, str>>,
  #[serde(default)]
  pub config: HashMap<Cow<'p, str>, Cow<'p, str>>,
  #[serde(default)]
  pub dependencies: HashMap<Cow<'p, str>, DepVersion<'p>>,
  #[serde(rename = "devDependencies")]
  pub dev_dependencies: HashMap<Cow<'p, str>, DepVersion<'p>>,
  #[serde(rename = "peerDependencies")]
  pub peer_dependencies: HashMap<Cow<'p, str>, DepVersion<'p>>,
  #[serde(rename = "bundledDependencies")]
  pub bundled_dependencies: HashMap<Cow<'p, str>, DepVersion<'p>>,
  #[serde(rename = "optionalDependencies")]
  pub optional_dependencies: HashMap<Cow<'p, str>, DepVersion<'p>>,
  pub engines: HashMap<Cow<'p, str>, Version>,
  #[serde(rename = "engineStrict")]
  // Deprecated in npm 3
  pub engine_strict: Option<bool>,
  #[serde(default)]
  pub os: Vec<Cow<'p, str>>,
  #[serde(default)]
  pub cpu: Vec<Cow<'p, str>>,
  #[serde(rename = "preferGlobal")]
  // Deprecated
  pub prefer_global: Option<bool>,
  #[serde(default)]
  pub private: bool,
  #[serde(rename = "publishConfig", default)]
  pub publish_config: HashMap<Cow<'p, str>, Cow<'p, str>>,
}

#[derive(Deserialize, Debug)]
pub enum Man<'p> {
  Multiple(Vec<Cow<'p, str>>),
  Single(Cow<'p, str>),
}

#[derive(Deserialize, Debug)]
pub enum Bin<'p> {
  Map(HashMap<Cow<'p, str>, Cow<'p, Path>>),
  Single(Cow<'p, Path>),
}

#[derive(Deserialize, Debug)]
pub enum Bugs<'p> {
  OneLine(Cow<'p, str>),
  Bug(Bug<'p>),
}

#[derive(Deserialize, Debug)]
pub struct Bug<'p> {
  pub url: Cow<'p, str>,
  pub email: Cow<'p, str>,
}

#[derive(Deserialize, Debug)]
pub enum PersonFormat<'p> {
  OneLine(Cow<'p, str>),
  Person(Person<'p>),
}

#[derive(Deserialize, Debug)]
pub struct Person<'p> {
  pub name: Cow<'p, str>,
  pub email: Option<Cow<'p, str>>,
  pub url: Option<Cow<'p, str>>,
}

#[derive(Deserialize, Debug)]
pub struct Directories<'p> {
  pub lib: Option<Cow<'p, Path>>,
  pub bin: Option<Cow<'p, Path>>,
  pub man: Option<Cow<'p, Path>>,
  pub doc: Option<Cow<'p, Path>>,
  pub example: Option<Cow<'p, Path>>,
  pub test: Option<Cow<'p, Path>>,
}

#[derive(Deserialize, Debug)]
pub enum Repository<'p> {
  Single(Cow<'p, str>),
  Complex(Repo<'p>),
}

#[derive(Deserialize, Debug)]
pub struct Repo<'p> {
  #[serde(rename = "type")]
  pub ty: Option<Cow<'p, Path>>,
  pub url: Option<Cow<'p, Path>>,
  pub directory: Option<Cow<'p, Path>>,
}

#[derive(Deserialize, Debug)]
pub enum DepVersion<'p> {
  Version(Version),
  GitHub(Cow<'p, str>),
  Url(SerdeUrl),
}
