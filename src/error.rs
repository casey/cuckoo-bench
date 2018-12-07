use crate::common::*;

#[derive(Debug)]
pub enum Error {
  Clone {
    git_error: git::Error,
  },
  Dirty {
    repository_path: String,
  },
  Io {
    io_error: io::Error,
    path: String,
  },
  Revparse {
    git_error: git::Error,
    spec: String,
  },
  Reset {
    git_error: git::Error,
    spec: String,
  },
  SetHead {
    git_error: git::Error,
    spec: String,
  },
  MakeInvocation {
    io_error: io::Error,
    path: String,
  },
  MakeStatus {
    exit_status: ExitStatus,
    path: String,
  },
  SolverInvocation {
    io_error: io::Error,
    path: String,
  },
  SolverStatus {
    exit_status: ExitStatus,
    path: String,
  },
  SolverDecode {
    from_utf8_error: FromUtf8Error,
  },
  RunParse {
    solver_output: String,
  },
  GraphParse {
    graph_lines: Vec<String>,
  },
  Write {
    io_error: io::Error,
    path: String,
  },
}
