use anyhow::{bail, Result, anyhow};
use aoc_2020::{parse_input};
use std::collections::HashSet;
use nom::lib::std::collections::HashMap;

struct BagRule {
    bag_type: String,
    can_contain: Vec<BagQty>
}

impl BagRule {
    fn parse(line: &str) -> Result<BagRule> {
        match line.split(" bags contain ").collect::<Vec<&str>>().as_slice() {
            [container_desc, rest] => {
                Ok(BagRule {
                    bag_type: container_desc.to_string(),
                    can_contain: BagQty::parse(rest)?
                })
            },
            _ => bail!("Unparseable rule")
        }
    }
}

struct BagQty {
    bag_type: String,
    qty: usize
}
impl BagQty {
    fn parse(rule: &str) -> Result<Vec<BagQty>> {
        if rule == "no other bags." {
            return Ok(Vec::new())
        }
        let mut quantities = Vec::new();
        for fragment in rule.split(", ")  {
            let tokens: Vec<&str> = fragment.split_whitespace().collect();
            if let [qty, adj, color, _bag] = tokens.as_slice() {
                let qty : usize = qty.parse()?;
                quantities.push(BagQty {
                    bag_type: format!("{} {}", adj, color),
                    qty
                });
            } else {
                bail!("Bad rule fragment: {}", fragment)
            }
        }
        Ok(quantities)
    }
}

struct RuleSystem {
    bag_can_contain: HashMap<String, Vec<BagQty>>
}
impl RuleSystem {
    fn from_axioms(axioms: Vec<BagRule>) -> RuleSystem {
        let mut rules = HashMap::new();
        for rule in axioms {
            rules.insert(rule.bag_type, rule.can_contain);
        }
        let missing : Vec<String> = rules.values().flatten().map(|r| r.bag_type.clone()).filter(|bt| !rules.contains_key(bt)).collect();
        for bag_type in missing {
            rules.insert(bag_type, Vec::new());
        }
        RuleSystem { bag_can_contain: rules }
    }
    fn invert(&self) -> InvertedRuleSystem {
        let mut inverted = HashMap::new();

        for (key, values) in self.bag_can_contain.iter() {
            for value in values {
                inverted.entry(value.bag_type.clone()).or_insert_with(|| HashSet::new()).insert(key.clone());
            }
        }
        for key in self.bag_can_contain.keys().filter(|k| !inverted.contains_key(*k)).collect::<Vec<&String>>() {
            inverted.insert(key.clone(), HashSet::new());
        }
        InvertedRuleSystem { bag_contained_by: inverted }
    }
    fn contained_count(&self, bag_type: &str) -> Result<usize> {
       self.contained_count_inner(&bag_type.to_string(), &mut HashSet::new())
    }

    fn contained_count_inner(&self, bag_type: &String, stack: &mut HashSet<String>) -> Result<usize> {
        if stack.contains(bag_type) {
            bail!("Cycle detected");
        }
        stack.insert(bag_type.clone());
        let mut count= 0;
        let contained_bags = self.bag_can_contain.get(bag_type).ok_or(anyhow!("Unknown bag type"))?;
        for contained in contained_bags {
            count += contained.qty;
            count += contained.qty * self.contained_count_inner(&contained.bag_type, stack)?;
        }
        stack.remove(bag_type);
        Ok(count)
    }
}

struct InvertedRuleSystem {
    bag_contained_by: HashMap<String, HashSet<String>>
}

impl InvertedRuleSystem {
    // todo: handle missing bag_type and cycles in the graph
    fn eventual_containers(&self, bag_type: &str) -> HashSet<String> {
        let mut results = HashSet::new();
        let mut search_queue = Vec::new();
        search_queue.push(bag_type.to_string());
        while !search_queue.is_empty() {
            let search = search_queue.pop().unwrap();
            for container in self.bag_contained_by.get(&search).expect("System wasn't symmetric") {
                if !results.contains(container) {
                    results.insert(container.clone());
                    search_queue.push(container.clone());
                }
            }
        }
        results
    }
}

fn main() -> Result<()> {
    let input = parse_input(7, |line| BagRule::parse(line))?;

    let system = RuleSystem::from_axioms(input);
    let inverted = system.invert();
    let inverted_count = inverted.eventual_containers("shiny gold").len();
    println!("shiny gold can be contained by {} bag types", inverted_count);

    let contained_count = system.contained_count("shiny gold")?;
    println!("shiny gold contains {} bags", contained_count);

    Ok(())
}