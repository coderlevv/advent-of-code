use crate::workflow::{Rule, Output, Workflow};
use std::collections::HashMap;
use regex::Regex;

type Part = HashMap<char, i32>;

pub fn solve(input: &[String]) {
    //let mut read_parts = false;
    let re_workflow = Regex::new(r"(\w+)\{(.+)\}").unwrap();
    let re_rule = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
    let re_part = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    for line in input {
        if let Some(wf_capts) = re_workflow.captures(line) {
            //println!("{:?}", capts);
            let key = wf_capts[1].to_string();
            let rules_str: Vec<_> = wf_capts[2].split(',').map(|s| s.trim()).collect();
            let mut rules: Vec<Rule> = vec![];
            for (idx, rule) in rules_str.iter().enumerate() {
                if idx < rules_str.len()-1 {
                    if let Some(rule_capts) = re_rule.captures(rule) {
                        let var = rule_capts[1].chars().next().unwrap();
                        let fct = rule_capts[2].chars().next().unwrap();
                        let val = rule_capts[3].parse::<i32>().unwrap();
                        let rule_out = match &rule_capts[4] {
                            "A" => Output::Accept,
                            "R" => Output::Reject,
                            out => Output::Workflow(out.to_string())
                        };
                        rules.push(Rule::new(var, fct, val, rule_out));
                    }
                } else {
                    let wf_out = match rule.to_owned() {
                        "A" => Output::Accept,
                        "R" => Output::Reject,
                        out => Output::Workflow(out.to_string())
                    };
                    workflows.insert(key.clone(), Workflow::new(rules.clone(), wf_out));  
                }
            }
        }
        if let Some(part_capts) = re_part.captures(line) {
            let x = part_capts[1].parse::<i32>().unwrap();
            let m = part_capts[2].parse::<i32>().unwrap();
            let a = part_capts[3].parse::<i32>().unwrap();
            let s = part_capts[4].parse::<i32>().unwrap();
            let part: Part = HashMap::from([('x', x), ('m', m), ('a', a), ('s', s)]);
            parts.push(part);
        }
    }

    let mut outputs: Vec<Output> = vec![];
    for part in &parts {
        let mut wf = workflows.get("in").unwrap();
        loop {
            let out = wf.run(part);
            match out {
                Output::Accept => { outputs.push(out); break; },
                Output::Reject => { outputs.push(out); break; },
                Output::Workflow(new_wf) => wf = workflows.get(&new_wf).unwrap()
            }
        }
    }
    let mut res: i32 = 0;
    for (idx, out) in outputs.iter().enumerate() {
        if *out == Output::Accept {
            res += parts[idx].values().sum::<i32>();
        }
    }
    println!("{}", res);
}