
use lsystem::{Axiom, Rules, LSystem};

pub fn main() {
  let system = {
    // rules for 
    let axiom = Axiom::from(&['A', '-', 'B', '-', 'B'][..]);
    let mut rules = Rules::new();
    rules.introduce('A', ['A', '-', 'B', '+', 'A', '+', 'B', '-', 'A']);
    rules.introduce('B', vec!['B', 'B']);
    LSystem::new(axiom, rules)
  };

  for (n, ns) in system.enumerate().skip(6).take(1) {
    println!("{n} = {}", ns.into_iter().collect::<String>());
  }
}