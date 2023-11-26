
//use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
pub struct IoLoad {
  pub read_io: f64,
  pub read_merges: f64,
  pub read_sectors: f64,
  pub read_ticks: f64,
  pub write_io: f64,
  pub write_merges: f64,
  pub write_sectors: f64,
  pub write_ticks: f64,
  pub in_flight: f64,
  pub io_ticks: f64,
  pub time_in_queue: f64,
  pub discard_io: Option<f64>,
  pub discard_merged: Option<f64>,
  pub discard_sectors: Option<f64>,
  pub discard_tickks: Option<f64>
}

impl IoLoad {
    /*
  #[cfg(unix)]
  pub fn snapshot() -> HashMap<String, IoLoad> {
    //use time::precise_time_s;

    let mut result = HashMap::new();
    if let Ok(dir) = std::fs::read_dir("/sys/block/") {
      for entry in dir.flatten() {
        let stat = entry.path().join(path);
        let mut s = String::new();
        if File::open(stat)
          .and_then(|mut f| f.read_to_string(&mut s))
          .is_err() {
            continue;
        }
        let parts = s
          .split_whitespace()
          .map(|w| w.parse().unwrap_or_default())
          .collect::<Vec<f64>>();
        if parts.len() < 1 {
          continue;
        }
        let load = IoLoad {
          read_io: parts[0],
          read_merges: parts[1],
          read_sectors: parts[2],
          read_ticks: parts[3],
          write_io: parts[4],
          write_merges: parts[5],
          write_sectors: parts[6],
          write_ticks: parts[7],
          in_flight: parts[8],
          io_ticks: parts[9],
          time_in_queue: parts[10],
          discard_io: parts.get(11).cloned(),
          discard_merged: parts.get(12).cloned(),
          discard_sectors: parts.get(13).cloned(),
          discard_tickks: parts.get(14).cloned()
        };
        result.insert(format!("{:?}", entry.file_name()), load);
      }
    }
    result
  }
  */
}