mod algorithm;
mod common;
mod config;
mod error;
mod graph;
mod run;

use crate::common::*;

use git::{Repository, RepositoryState, ResetType};
use structopt::StructOpt;

const REPOSITORY_PATH: &str = "./cuckoo";
const SOLVER_BINARY: &str = "cuda29";

use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Error> {
  let config = Config::from_args();

  if Path::new(REPOSITORY_PATH).exists() {
    eprintln!("Cleaning up `{}`...", REPOSITORY_PATH);
    fs::remove_dir_all(REPOSITORY_PATH).map_err(|io_error| Error::Io {
      io_error,
      path: REPOSITORY_PATH.to_string(),
    })?;
  }

  eprintln!(
    "Cloning `{}` into `{}`...",
    config.repository, REPOSITORY_PATH
  );
  let repository = Repository::clone(&config.repository, REPOSITORY_PATH)
    .map_err(|git_error| Error::Clone { git_error })?;

  if repository.state() != RepositoryState::Clean {
    return Err(Error::Dirty {
      repository_path: REPOSITORY_PATH.to_string(),
    });
  }

  let spec = config.revision;

  eprintln!("Checking out `{}`...", spec);
  let revision = repository
    .revparse_single(&spec)
    .map_err(|git_error| Error::Revparse {
      git_error,
      spec: spec.clone(),
    })?;

  if spec != "master" {
    repository
      .set_head_detached(revision.id())
      .map_err(|git_error| Error::SetHead {
        git_error,
        spec: spec.clone(),
      })?;

    repository
      .reset(&revision, ResetType::Hard, None)
      .map_err(|git_error| Error::Reset {
        git_error,
        spec: spec.clone(),
      })?;
  }

  for algorithm in Algorithm::all() {
    eprintln!("Building {}29 cuda solver...", algorithm);

    let current_dir = format!("{}/src/{}", REPOSITORY_PATH, algorithm);

    let exit_status = Command::new("make")
      .arg(SOLVER_BINARY)
      .current_dir(&current_dir)
      .status()
      .map_err(|io_error| Error::MakeInvocation {
        io_error,
        path: current_dir.clone(),
      })?;

    if !exit_status.success() {
      return Err(Error::MakeStatus {
        exit_status,
        path: current_dir.clone(),
      });
    }
  }

  for &algorithm in Algorithm::all() {
    eprintln!("Benchmarking {} solver...", algorithm);

    let solver = format!("{}/src/{}/{}", REPOSITORY_PATH, algorithm, SOLVER_BINARY);

    let output = Command::new(&solver)
      .arg("-r")
      .arg("10")
      .output()
      .map_err(|io_error| Error::SolverInvocation {
        io_error,
        path: solver.clone(),
      })?;

    if !output.status.success() {
      return Err(Error::SolverStatus {
        exit_status: output.status,
        path: solver.clone(),
      });
    }

    let output = String::from_utf8(output.stdout)
      .map_err(|from_utf8_error| Error::SolverDecode { from_utf8_error })?;

    let run = Run::from_solver_output(
      &config.repository,
      &spec,
      &revision.id().to_string(),
      algorithm,
      29,
      0,
      &output,
    )?;

    let yaml = serde_yaml::to_string(&run).unwrap();

    let seconds_since_epoch = {
      let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
      elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000
    };

    let path = format!("runs/{}-{}.yaml", seconds_since_epoch, algorithm.name());

    fs::write(&path, &yaml).map_err(|io_error| Error::Write {
      io_error,
      path: path.to_string(),
    })?;
  }

  Ok(())
}
