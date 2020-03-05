use jane_eyre::Result;
use serde::Deserialize;
use std::borrow::Cow;
use structopt::StructOpt;
use tracing::{
  error,
  instrument,
};
use tracing_subscriber::FmtSubscriber;

#[derive(StructOpt, Debug)]
#[structopt(name = "kt")]
pub enum Opt {
  /// Install your deps
  Install(Install),
}

#[derive(StructOpt, Debug)]
pub struct Install {}

fn main() {
  if let Err(e) = run() {
    error!("{}", e)
  }
}

#[instrument(level = "debug")]
fn run() -> Result<()> {
  let subscriber = FmtSubscriber::new();
  tracing::subscriber::set_global_default(subscriber)?;
  let opt = Opt::from_args();
  match opt {
    Opt::Install(_install) => {
      let pj =
        serde_json::from_slice::<PackageJson>(&std::fs::read("package.json")?)?;
    }
  }
  Ok(())
}

#[derive(Deserialize, Debug)]
pub struct PackageJson<'p> {
  name: Option<Cow<'p, str>>,
  version: Option<Cow<'p, str>>,
  description: Option<Cow<'p, str>>,
  #[serde(default)]
  keywords: Vec<Cow<'p, str>>,
  homepage: Option<Cow<'p, str>>,
  bugs: Option<Bugs<'p>>,
  license: Option<Cow<'p, str>>,
  author: Option<PersonFormat<'p>>,
  #[serde(default)]
  contributors: Vec<Cow<'p, str>>,
  #[serde(default)]
  files: Vec<Cow<'p, str>>,
  main: Option<Cow<'p, str>>,
  browser: Option<Cow<'p, str>>,
}

#[derive(Deserialize, Debug)]
pub enum Bugs<'p> {
  OneLine(Cow<'p, str>),
  Bug(Bug<'p>),
}

#[derive(Deserialize, Debug)]
pub struct Bug<'p> {
  url: Cow<'p, str>,
  email: Cow<'p, str>,
}

#[derive(Deserialize, Debug)]
pub enum PersonFormat<'p> {
  OneLine(Cow<'p, str>),
  Person(Person<'p>),
}

#[derive(Deserialize, Debug)]
pub struct Person<'p> {
  name: Cow<'p, str>,
  email: Option<Cow<'p, str>>,
  url: Option<Cow<'p, str>>,
}
