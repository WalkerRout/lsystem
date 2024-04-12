
use std::mem;
use std::hash::Hash;
use std::collections::HashMap;

pub trait Alphabet: Hash + Eq {}

impl<T> Alphabet for T where T: Hash + Eq {}

#[derive(Debug, Default, Clone)]
pub struct Rules<A> {
  pub rules: HashMap<A, Vec<A>>,
}

impl<A> Rules<A>
  where A: Alphabet {
  pub fn introduce(&mut self, variable: A, productions: impl Into<Vec<A>>) {
    self.rules.entry(variable).or_insert_with(|| productions.into());
  }
}

impl<A> PartialEq for Rules<A>
  where A: Alphabet {
  fn eq(&self, other: &Self) -> bool {
    self.rules == other.rules
  }
}

#[derive(Debug, Default, Clone)]
pub struct Axiom<A> {
  pub symbols: Vec<A>,
}

impl<A> From<&[A]> for Axiom<A>
  where A: Alphabet + Clone {
  fn from(l: &[A]) -> Self {
    assert!(!l.is_empty(), "axiom cannot have length 0");
    Self { symbols: l.to_owned(), }
  }
}

impl<A> PartialEq for Axiom<A>
  where A: Alphabet {
  fn eq(&self, other: &Self) -> bool {
    self.symbols == other.symbols
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
    Self { axiom, rules, state, }
  }
}

impl<A> Iterator for LSystem<A>
  where A: Alphabet + Clone + Send + Sync {
  type Item = Vec<A>;

  fn next(&mut self) -> Option<Vec<A>> {
    self.state = mem::take(&mut self.state)
      .into_iter()
      .flat_map(|k| {
        self.rules.rules.get(&k).cloned().unwrap_or_else(|| vec![k])
      })
      .collect();
    // never ending iterator; could check if state changed and if not return none
    Some(self.state.clone())
  }
}

impl<A> PartialEq for LSystem<A>
  where A: Alphabet {
  fn eq(&self, other: &Self) -> bool {
    self.state == other.state &&
    self.axiom == other.axiom &&
    self.rules == other.rules
  }
}

#[cfg(test)]
mod tests {
  use rstest::*;
  use super::*;

  #[fixture]
  fn axiom_char_empty() -> Axiom<char> {
    Axiom {
      symbols: Vec::new(),
    }
  }

  #[fixture]
  fn axiom_char_a() -> Axiom<char> {
    let mut axiom = axiom_char_empty();
    axiom.symbols.push('a');
    axiom
  }

  #[fixture]
  fn rules_char_empty() -> Rules<char> {
    Rules {
      rules: HashMap::new(),
    }
  }

  #[fixture]
  fn rules_char_a_to_b() -> Rules<char> {
    let mut rules = rules_char_empty();
    rules.rules.entry('a').or_insert_with(|| vec!['b']);
    rules
  }

  #[fixture]
  fn rules_char_a_to_b_b_to_ab() -> Rules<char> {
    let mut rules = rules_char_empty();
    rules.rules.entry('a').or_insert_with(|| vec!['b']);
    rules.rules.entry('b').or_insert_with(|| vec!['a', 'b']);
    rules
  }

  #[fixture]
  fn l_system_char_empty() -> LSystem<char> {
    LSystem {
      axiom: axiom_char_empty(),
      rules: rules_char_empty(),
      state: Vec::new(),
    }
  }

  #[fixture]
  fn l_system_char_fibonacci() -> LSystem<char> {
    let mut system = l_system_char_empty();
    system.axiom = axiom_char_a();
    system.rules = rules_char_a_to_b_b_to_ab();
    system.state = system.axiom.symbols.clone();
    system
  }

  mod axiom {
    use super::*;

    #[rstest]
    #[should_panic]
    #[case(&[])]
    #[case(&['a'])]
    #[case(&['a', 'b'])]
    #[case(&['a', 'b', 'c'])]
    fn from_slice(
      #[case] slice: &[char],
    ) {
      let axiom = Axiom::from(slice);
      assert_eq!(&axiom.symbols, slice);
    }
  }

  mod rules {
    use super::*;

    #[rstest]
    #[case(rules_char_empty(), 'a', &[], &[])]
    #[case(rules_char_empty(), 'a', &['a'], &['a'])]
    #[case(rules_char_empty(), 'a', &['a', 'b'], &['a', 'b'])]
    #[case(rules_char_empty(), 'b', &['a', 'b'], &['a', 'b'])]
    #[case(rules_char_a_to_b(), 'a', &['a', 'b', 'c'], &['b'])]
    #[case(rules_char_a_to_b_b_to_ab(), 'b', &['c'], &['a', 'b'])]
    fn introduce(
      #[case] mut rules: Rules<char>,
      #[case] pred: char,
      #[case] products: &[char],
      #[case] expected_products: &[char],
    ) {
      rules.introduce(pred, products);
      assert_eq!(rules.rules.get(&pred).unwrap(), expected_products);
    }
  }

  mod l_system {
    use super::*;

    #[rstest]
    #[case(axiom_char_empty(), rules_char_empty(), l_system_char_empty())]
    #[case(axiom_char_a(), rules_char_a_to_b_b_to_ab(), l_system_char_fibonacci())]
    fn new(
      #[case] axiom: Axiom<char>,
      #[case] rules: Rules<char>,
      #[case] expected_system: LSystem<char>,
    ) {
      let system = LSystem::new(axiom, rules);
      assert_eq!(system, expected_system);
    }

    // iterator never returns None; ensure it
    #[rstest]
    #[case(l_system_char_empty(), 0, &[])]
    #[case(l_system_char_empty(), 1, &[])]
    #[case(l_system_char_empty(), 2, &[])]
    #[case(l_system_char_empty(), 4, &[])]
    #[case(l_system_char_fibonacci(), 0, &['b'])]
    #[case(l_system_char_fibonacci(), 1, &['a', 'b'])]
    #[case(l_system_char_fibonacci(), 2, &['b', 'a', 'b'])]
    #[case(l_system_char_fibonacci(), 4, &['b', 'a', 'b', 'a', 'b', 'b', 'a', 'b'])]
    fn next(
      #[case] system: LSystem<char>,
      #[case] skip_iters: usize,
      #[case] expected_next_state: &[char],
    ) {
      let next = system.skip(skip_iters).next().unwrap();
      assert_eq!(next, expected_next_state);
    }
  }
}