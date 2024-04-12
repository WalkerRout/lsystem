
use lsystem::{Axiom, Rules, LSystem};

fn parse_luigi(luigi_source: &str) -> LSystem<char> {
  let mut chars = luigi_source.chars();
  chars.next(); // consume ';'
  let mut axiom = Axiom::default();
  axiom.symbols.extend(chars.by_ref().take_while(|c| *c != ';'));
  let mut rules = Rules::default();
  // single predecessor -> chars.next() consumes ';' immediately after
  while let Some(ch) = chars.next() {
    chars.next(); // consume ';'
    let prods: Vec<_> = chars
      .by_ref()
      .take_while(|c| *c != ';')
      .collect();
    rules.introduce(ch, prods);
  }
  LSystem::new(axiom, rules)
}

pub fn main() {
  let system = parse_luigi(";a;a;b;b;ab;");

  for (n, ns) in system.enumerate().skip(6).take(1) {
    println!("{} = {}", n+1, ns.into_iter().collect::<String>());
  }
}