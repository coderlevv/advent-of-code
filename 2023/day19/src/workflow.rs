use std::collections::HashMap;

type Part = HashMap<char, i32>;

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Accept,
    Reject,
    Workflow(String)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub var: char,
    pub fct: char,
    pub val: i32,
    pub out: Output
}

impl Rule {
    pub fn new(var: char, fct: char, val: i32, out: Output) -> Self {
        Self {
            var,
            fct,
            val,
            out
        }
    }

    pub fn eval(&self, part: &Part) -> Option<Output> {
        let part_val = part.get(&self.var).unwrap();
        match self.fct {
            '<' => if *part_val < self.val { Some(self.out.clone()) } else { None },
            '>' => if *part_val > self.val { Some(self.out.clone()) } else { None },
            _ => panic!("Unknown function!")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Workflow {
    pub rules: Vec<Rule>,
    pub out: Output
}

impl Workflow {
    pub fn new(rules: Vec<Rule>, out: Output) -> Self {
        Self {
            rules,
            out
        }
    }

    pub fn run(&self, part: &Part) -> Output {
        let out = self.out.clone();
        for r in &self.rules {
            if let Some(out) = r.eval(part) {
                return out;
            }
        }
        out
    }
}

#[test]
fn rule_test() {
    let part: Part = HashMap::from([('x', 787), ('m', 2655), ('a', 1222), ('s', 2876)]);
    let rule: Rule = Rule::new('a', '>', 1223, Output::Workflow("qkq".to_string()));
    assert_eq!(rule.eval(&part), Some(Output::Workflow("qkq".to_string())));
    
    let part: Part = HashMap::from([('x', 787), ('m', 2655), ('a', 1222), ('s', 2876)]);
    let rule: Rule = Rule::new('a', '>', 1222, Output::Workflow("qkq".to_string()));
    assert_eq!(rule.eval(&part), None);

    let part: Part = HashMap::from([('x', 787), ('m', 2655), ('a', 1222), ('s', 2876)]);
    let rule: Rule = Rule::new('a', '>', 1221, Output::Workflow("qkq".to_string()));
    assert_eq!(rule.eval(&part), None);
}