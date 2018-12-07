use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cuckoo-bench")]
pub struct Config {
  #[structopt(
    long = "repository",
    default_value = "https://github.com/tromp/cuckoo.git"
  )]
  pub repository: String,
  #[structopt(long = "revision", default_value = "master")]
  pub revision: String,
}
