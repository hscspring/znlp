use std::collections::HashMap;
use std::vec::Vec;
use core::cmp::max;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Entry {
  start: String,
  end: String,
  weight: i32,
}

impl Entry {
  pub fn new(start: &str, end: &str, weight: i32) -> Entry {
    Entry { start: start.to_owned(), end: end.to_owned(), weight: weight }
  }
}

#[derive(Debug)]
struct UndirectWeightedGraph {
  d: f32,
  graph: HashMap<String, Vec<Entry>>,
}

impl UndirectWeightedGraph {
  pub fn new(d: f32) -> UndirectWeightedGraph {
    UndirectWeightedGraph { d: d, graph: HashMap::new() }
  }
  pub fn add_edge(mut self, entry: Entry) -> UndirectWeightedGraph {
    match self.graph.get_mut(&entry.start) {
      Some(vec) => {
        vec.push(entry.clone());
      },
      None => {
        self.graph.insert(entry.start.clone(), [entry.clone()].to_vec());
      }
    };
    match self.graph.get_mut(&entry.end) {
      Some(vec) => {
        vec.push(entry);
      },
      None => {
        self.graph.insert(entry.end.clone(), [entry].to_vec());
      }
    }
    self
  }
  pub fn len(&self) -> usize {
    self.graph.len()
  }
  pub fn rank(&self) -> HashMap<String, f32> {
    let mut ws: HashMap<String, f32> = HashMap::new();
    let mut out_sum: HashMap<String, f32> = HashMap::new();

    let wsdef = 1.0 / max(1, self.len()) as f32;
    for (n, out) in self.graph.iter() {
      ws.insert(n.to_owned(), wsdef);
      let mut sum: i32 = 0;
      for elem in out.iter() {
        sum += elem.weight;
      }
      out_sum.insert(n.to_owned(), sum as f32);
    }

    let mut sorted_keys = Vec::new();
    for (key, _) in self.graph.iter() {
      sorted_keys.push(key);
    };
    sorted_keys.sort();

    for _i in 0..10 {
      for n in sorted_keys.iter() {
        let mut s: f32 = 0.0;
        let entries: &Vec<Entry> = self.graph.get(n.to_owned()).unwrap();
        for e in entries.iter() {
          s += e.weight as f32 / out_sum.get(&e.end).unwrap() * ws.get(&e.end).unwrap();
          if let Some(val) = ws.get_mut(n.to_owned()) {
            *val = (1.0 - self.d) + self.d * s;
          }
        }
      }
    }

    let min_wei = ws.values().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_wei = ws.values().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    for (_n, wei) in ws.iter_mut() {
      *wei = (*wei - min_wei / 10.0) / (max_wei - min_wei / 10.0);
    }
    ws
  }
}

impl Default for UndirectWeightedGraph {
  fn default() -> Self {
    UndirectWeightedGraph::new(0.85)
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entry() {
        let entry = Entry::new("我", "喜欢", 3);
        assert_eq!(entry.start, String::from("我"));
        assert_eq!(entry.weight, 3);
    }

    #[test]
    fn undirect_weighted_graph_default() {
      let graph = UndirectWeightedGraph::default();
      assert_eq!(graph.d, 0.85);
    }

    #[test]
    fn graph_add_edge() {
      let mut g = UndirectWeightedGraph::default();
      let entry = Entry::new("我", "喜欢", 3);
      g = g.add_edge(entry.clone());
      assert_eq!(g.len(), 2);
      assert_eq!(g.graph.get("我"), Some(&[entry].to_vec()));
    }

    #[test]
    fn graph_rank() {
      let mut g = UndirectWeightedGraph::default();
      let entry = Entry::new("我", "喜欢", 3);
      g = g.add_edge(entry);
      let entry = Entry::new("喜欢", "你", 5);
      g = g.add_edge(entry);
      let ranked = g.rank();
      assert!(ranked.get("我").unwrap() - 0.409 < 0.1);
      assert!(ranked.get("喜欢").unwrap() - 1.0 < 0.1);
      assert!(ranked.get("你").unwrap() - 0.659 < 0.1);
    }
}
