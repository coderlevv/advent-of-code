use crate::parse;
use std::collections::HashMap;
use crate::workflow::{Output, Workflow};

type Range = Option<(i32, i32)>;

pub fn solve(input: &[String]) {
    let (workflows, _) = parse::parse(input); 
    let mut counts: u64 = 0;
    // x, m, a, s
    let ranges: [Range; 4] = [
        Some((1, 4000)),
        Some((1, 4000)),
        Some((1, 4000)),
        Some((1, 4000))
    ];
    let wf = workflows.get(&"in".to_string()).unwrap();
    count(ranges, wf, &workflows, &mut counts);
    println!("{:?}", counts);
}

pub fn count(
    ranges: [Range; 4],
    workflow: &Workflow,
    workflows: &HashMap<String, Workflow>,
    counts: &mut u64
) {
    let mut ranges = ranges;
    let mut accept: Range;
    let mut reject: Range;
    for rule in &workflow.rules {
        let idx = match rule.var {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("Unknown var!")
        };
        if let Some(range) = ranges[idx] {
            (accept, reject) = apply_rule(&range, &rule.val, &rule.fct);
            ranges[idx] = accept;
            match &rule.out {
                Output::Accept => { *counts += combinations(&ranges); },
                Output::Reject => { },
                Output::Workflow(next_wf) => {
                    let wf = workflows.get(next_wf).unwrap();
                    count(ranges, wf, workflows, counts);
                }
            }
            ranges[idx] = reject;
        }
    }
    match &workflow.out {
        Output::Accept => { *counts += combinations(&ranges) },
        Output::Reject => { },
        Output::Workflow(next_wf) => {
            let wf = workflows.get(next_wf).unwrap();
            count(ranges, wf, workflows, counts);
        }
    }
}

fn combinations(ranges: &[Range]) -> u64 {
    let mut combis: u64 = 1;
    for range in ranges.iter().flatten() {
        combis *= range.1 as u64 - range.0 as u64 + 1;
    }
    combis
}

fn apply_rule(range: &(i32, i32), val: &i32, fct: &char) -> (Range, Range) {
    let accept: Range;
    let reject: Range;
    match fct {
        '<' => {
            if range.0 >= *val {
                accept = None;
                reject = Some(*range);
            }
            else if range.1 < *val {
                reject = None;
                accept = Some(*range);
            }
            else {
                accept = Some((range.0, *val-1));
                reject = Some((*val, range.1));
            }
        },
        '>' => {
            if range.1 <= *val {
                accept = None;
                reject = Some(*range);
            }
            else if range.0 > *val {
                reject = None;
                accept = Some(*range); }
            else {
                accept = Some((*val+1, range.1));
                reject = Some((range.0, *val));
            }
        },
        _ => panic!("Unknown function!")
    }
    (accept, reject)
}

#[test]
fn apply_test() {
    let range: (i32, i32) = (1, 4000);
    let val = 1351;
    let fct = '<';
    assert_eq!(apply_rule(&range, &val, &fct), (Some((1, 1350)), Some((1351, 4000))));
}