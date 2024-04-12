
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;

use rayon::prelude::*;

pub trait Alphabet: Hash + Eq {}

impl<T> Alphabet for T where T: Hash + Eq {}

#[derive(Debug, Default, Clone)]
pub struct Rules<A> {
  pub rules: HashMap<A, Vec<A>>,
}

impl<A> Rules<A>
  where A: Alphabet {
  #[allow(clippy::must_use_candidate)]
  pub fn new() -> Self {
    Self {
      rules: HashMap::new(),
    }
  }

  pub fn introduce(&mut self, variable: A, productions: impl Into<Vec<A>>) {
    self.rules.entry(variable).or_insert_with(|| productions.into());
  }
}

impl<A> fmt::Display for Rules<A>
  where A: Alphabet + fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (k, v) in &self.rules {
      writeln!(f, "{} -> {}",
        k, v.iter().map(ToString::to_string).collect::<String>())?;
    }
    Ok(())
  }
}

#[derive(Debug, Default, Clone)]
pub struct Axiom<A> {
  pub symbols: Vec<A>,
}

impl<A> Axiom<A>
  where A: Alphabet {
  #[allow(clippy::must_use_candidate)]
  pub const fn new() -> Self {
    Self {
      symbols: Vec::new(),
    }
  }
}

impl<A> From<&[A]> for Axiom<A>
  where A: Alphabet + Clone {
  fn from(l: &[A]) -> Self {
    Self {
      symbols: l.to_owned(),
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct LSystem<A> {
  pub axiom: Axiom<A>,
  pub rules: Rules<A>,
  pub state: Vec<A>,
}

impl<A> LSystem<A>
  where A: Alphabet + Clone {
  #[allow(clippy::must_use_candidate)]
  pub fn new(axiom: Axiom<A>, rules: Rules<A>) -> Self {
    let state = axiom.symbols.clone();
    Self {
      axiom,
      rules,
      state,
    }
  }
}

impl<A> Iterator for LSystem<A>
  where A: Alphabet + Clone + Send + Sync {
  type Item = Vec<A>;

  fn next(&mut self) -> Option<Vec<A>> {
    self.state = std::mem::take(&mut self.state)
      .into_par_iter()
      .flat_map(|k| {
        self.rules.rules.get(&k).cloned().unwrap_or_else(|| vec![k])
      })
      .collect();
    Some(self.state.clone())
  }
}