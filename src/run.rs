use crate::common::*;

#[derive(Debug, Serialize)]
pub struct Run {
  repository: String,
  revision: String,
  commit: String,

  algorithm: Algorithm,
  size: u64,
  edge_expansion_round: u64,

  device: String,
  description: String,
  memory: String,
  solutions_found: u64,
  graphs: Vec<Graph>,
}

impl Run {
  pub fn from_solver_output(
    repository: &str,
    revision: &str,
    commit: &str,

    algorithm: Algorithm,
    size: u64,
    edge_expansion_round: u64,

    solver_output: &str,
  ) -> Result<Run, Error> {
    let lines = solver_output.lines().collect::<Vec<&str>>();

    let run_parse_error = || Error::RunParse {
      solver_output: solver_output.to_string(),
    };

    if lines.len() < 4 {
      return Err(run_parse_error());
    }

    let device = lines[0].to_string();
    let description = lines[1].to_string();
    let memory = lines[2].to_string();
    let solutions_found = lines
      .last()
      .unwrap()
      .split(' ')
      .next()
      .ok_or_else(run_parse_error)?
      .parse()
      .map_err(|_| run_parse_error())?;

    let graph_lines = &lines[3..lines.len() - 1];

    let mut current_graph_lines = Vec::new();
    let mut graphs = Vec::new();

    for (i, line) in graph_lines.iter().enumerate() {
      if !current_graph_lines.is_empty()
        && (i == graph_lines.len() - 1 || line.starts_with("nonce "))
      {
        graphs.push(Graph::from_graph_lines(&current_graph_lines)?);
        current_graph_lines.clear();
      }

      current_graph_lines.push(*line);
    }

    Ok(Run {
      repository: repository.to_string(),
      revision: revision.to_string(),
      commit: commit.to_string(),

      algorithm,
      size,
      edge_expansion_round,

      device,
      description,
      memory,
      graphs,
      solutions_found,
    })
  }
}
