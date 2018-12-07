use crate::common::*;

#[derive(Debug, Serialize)]
pub struct Graph {
  nonce: u64,
  output: String,
  solve_time_ms: u64,
  graphs_per_second: f64,
  solutions_found: u64,
}

impl Graph {
  pub fn from_graph_lines(graph_lines: &[&str]) -> Result<Graph, Error> {
    let graph_parse_error = || Error::GraphParse {
      graph_lines: graph_lines.iter().cloned().map(str::to_string).collect(),
    };

    if graph_lines.len() < 3 {
      return Err(graph_parse_error());
    }

    let nonce = graph_lines[0]
      .split(' ')
      .skip(1)
      .next()
      .ok_or_else(graph_parse_error)?
      .parse()
      .map_err(|_| graph_parse_error())?;

    let output = graph_lines.join("\n");

    let time_parts = graph_lines
      .iter()
      .filter(|line| line.contains("ms total "))
      .next()
      .ok_or_else(graph_parse_error)?
      .split(' ')
      .collect::<Vec<&str>>();

    let solve_time_ms = time_parts[time_parts.len() - 2]
      .parse()
      .map_err(|_| graph_parse_error())?;
    let graphs_per_second = 100.0 / solve_time_ms as f64;

    let solutions_found = graph_lines
      .iter()
      .filter(|line| line.starts_with("Solution "))
      .count() as u64;

    Ok(Graph {
      nonce,
      output,
      solve_time_ms,
      solutions_found,
      graphs_per_second,
    })
  }
}
