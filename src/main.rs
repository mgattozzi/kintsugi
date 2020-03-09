mod package;
mod view;

use jane_eyre::Result;
use package::PackageJson;
use reqwest::blocking::Client;
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
  /// View a package
  View(View),
}

#[derive(StructOpt, Debug)]
pub struct Install {}

#[derive(StructOpt, Debug)]
pub struct View {}

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
      let _pj =
        serde_json::from_slice::<PackageJson>(&std::fs::read("package.json")?)?;
    }
    Opt::View(_view) => {
      const REGISTRY: &str = "https://registry.npmjs.com/";
      let client = Client::new();
      let res: view::ViewPkg = client
        .get(&format!("{}/{}/latest", REGISTRY, "react"))
        .send()?
        .json()?;
      println!("{:?}", res);
    }
  }
  Ok(())
}
