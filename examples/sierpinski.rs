use lsystem::{Axiom, LSystem, Rules};

pub fn main() {
  let system = {
    // rules for
    let axiom = Axiom::from(&['A', '-', 'B', '-', 'B'][..]);
    let mut rules = Rules::default();
    rules.introduce('A', ['A', '-', 'B', '+', 'A', '+', 'B', '-', 'A']);
    rules.introduce('B', vec!['B', 'B']);
    LSystem::new(axiom, rules)
  };

  for (n, ns) in system.enumerate().skip(8).take(1) {
    println!("{} = {}", n + 1, ns.into_iter().collect::<String>());
  }
}
