use std::collections::HashMap;

use crate::{WorkflowResult, Part, Rating, parse_rating};


pub(crate) fn part_1(input: &str) {
    let (workflows, ratings) = parse_input(input);

    let mut total: usize = 0;

    for rating in ratings {
        let mut name = "in".to_string();
        //println!("{rating:?}");
        loop {
            match workflows.get(&name).unwrap().includes(&rating) {
                WorkflowResult::Accepted => {
                    total += rating.sum();
                    break;
                },
                WorkflowResult::Rejected => break,
                WorkflowResult::Workflow(new_name) => name = new_name,
                WorkflowResult::Pass => unreachable!("Shouldn't pass here"),
            }
        }
        //println!("{total}");
    }
    
    println!("{total}");
}

#[derive(Debug)]
struct Rule {
    part: Part,
    min: usize,
    max: usize,
    result: WorkflowResult,
}

impl Rule {
    fn new(part: &str, min: &str, max: &str, result: &str) -> Self {
        Self {
            part: Part::new(part),
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            result: WorkflowResult::new(result)
        }
    }

    fn includes(&self, rating: &Rating) -> WorkflowResult {
        let value = match self.part {
            Part::X => rating.x,
            Part::M => rating.m,
            Part::A => rating.a,
            Part::S => rating.s,
        };

        if (value > self.min) & (value < self.max) {
            self.result.clone()
        }
        else {
            WorkflowResult::Pass
        }

    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    result: WorkflowResult,
}

impl Workflow {
    fn new() -> Self {
        Self { rules: Vec::new(), result: WorkflowResult::Rejected }
    }
    
    fn includes(&self, rating: &Rating) -> WorkflowResult {
        //println!("Check {}", self.name);
        for rule in &self.rules {
            //println!("\t{rule:?}");
            match rule.includes(&rating) {
                WorkflowResult::Pass => (),
                result => return result,
            }
        }
        self.result.clone()
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Rating>) {
    let input_trimmed = input.replace(' ', "");
    let (work_lines, part_lines) = if input_trimmed.contains("\n\n") {
        input_trimmed.split_once("\n\n").unwrap()
    } else {
        input_trimmed.split_once("\r\n\r\n").unwrap()
    };

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for line in work_lines.lines() {
        if line.trim().is_empty() {
            break
        }
        let (name, rest) = line.split_once('{').unwrap();
        let mut workflow = Workflow::new();
        let rules = &rest[..rest.len()-1];
        for rule in rules.split(',') {
            if rule.contains(':') {
                workflow.rules.push(parse_rule(rule));
            }
            else {
                workflow.result = WorkflowResult::new(rule);
            }
        }
        workflows.insert(name.to_string(), workflow);
    }
    
    let mut ratings: Vec<Rating> = Vec::new();

    for line in part_lines.lines() {
        let mut rating = line.replace("{", "");
        rating = rating.replace("}", "");
        ratings.push(parse_rating(&rating));
    }

    (workflows, ratings)
}

fn parse_rule(rule: &str) -> Rule {
    let (condition, result) = rule.split_once(':').unwrap();

    if condition.contains('<') {
        let (part, max) = condition.split_once('<').unwrap();
        Rule::new(part, &0.to_string(), max, result)
    }
    else {
        let (part, min) = condition.split_once('>').unwrap();
        Rule::new(part, min, &usize::MAX.to_string(), result)
    }
}
