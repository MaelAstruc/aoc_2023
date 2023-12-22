use std::{collections::HashMap, usize};

fn main() {
    assert_eq!(
        part_1(
            "px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}
            
            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}"
        ),
        19_114
    );

    assert_eq!(
        part_2(
            "px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}
            
            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}"
        ),
        167_409_079_868_000
    );
}

fn part_1(input: &str) -> usize {
    let (workflows, ratings) = parse_input(input);

    let mut total: usize = 0;

    for rating in ratings {
        let mut name = "in".to_string();
        loop {
            match workflows.get(&name).unwrap().includes(&rating) {
                WorkflowResult::Accepted => {
                    total += rating.sum();
                    break;
                }
                WorkflowResult::Rejected => break,
                WorkflowResult::Workflow(new_name) => name = new_name,
                WorkflowResult::Pass => unreachable!("Shouldn't pass here"),
            }
        }
    }

    println!("{total}");
    total
}

fn part_2(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    let mut total: usize = 0;

    let multi_rule = MultiRule::new();

    let multi_rules = multi_rule.divide(&workflows, "in", 0);

    for mr in &multi_rules {
        if mr.result == WorkflowResult::Accepted {
            total += mr.count_possibilities();
        }
    }

    println!("{total}");
    total
}

#[derive(Debug)]
enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    fn new(part: &str) -> Self {
        match part {
            "X" | "x" => Part::X,
            "M" | "m" => Part::M,
            "A" | "a" => Part::A,
            "S" | "s" => Part::S,
            _ => unreachable!("Unexpected part {part}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum WorkflowResult {
    Accepted,
    Rejected,
    Pass,
    Workflow(String),
}

impl WorkflowResult {
    fn new(result: &str) -> Self {
        match result {
            "A" => WorkflowResult::Accepted,
            "R" => WorkflowResult::Rejected,
            _ => WorkflowResult::Workflow(result.to_string()),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Lesser,
    Greater,
}

#[derive(Debug)]
struct Rule {
    part: Part,
    value: usize,
    operation: Operation,
    result: WorkflowResult,
}

impl Rule {
    fn new(part: &str, value: &str, operation: Operation, result: &str) -> Self {
        Self {
            part: Part::new(part),
            value: value.parse().unwrap(),
            operation,
            result: WorkflowResult::new(result),
        }
    }

    fn includes(&self, rating: &Rating) -> WorkflowResult {
        let value = match self.part {
            Part::X => rating.x,
            Part::M => rating.m,
            Part::A => rating.a,
            Part::S => rating.s,
        };

        let is_valid = match self.operation {
            Operation::Lesser => value < self.value,
            Operation::Greater => value > self.value,
        };

        if is_valid {
            self.result.clone()
        } else {
            WorkflowResult::Pass
        }
    }
}

#[derive(Debug, Clone)]
struct MultiRule {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
    result: WorkflowResult,
}

impl MultiRule {
    fn new() -> Self {
        Self {
            x: (0, 4001),
            m: (0, 4001),
            a: (0, 4001),
            s: (0, 4001),
            result: WorkflowResult::Rejected,
        }
    }

    fn check_valid(&self, rule: &Rule) -> bool {
        match (&rule.part, &rule.operation) {
            (Part::X, Operation::Lesser) => rule.value < self.x.1,
            (Part::X, Operation::Greater) => rule.value > self.x.0,
            (Part::M, Operation::Lesser) => rule.value < self.m.1,
            (Part::M, Operation::Greater) => rule.value > self.m.0,
            (Part::A, Operation::Lesser) => rule.value < self.a.1,
            (Part::A, Operation::Greater) => rule.value > self.a.0,
            (Part::S, Operation::Lesser) => rule.value < self.s.1,
            (Part::S, Operation::Greater) => rule.value > self.s.0,
        }
    }

    fn update_first_boundary(&mut self, rule: &Rule) {
        match (&rule.part, &rule.operation) {
            (Part::X, Operation::Lesser) => self.x.1 = rule.value,
            (Part::X, Operation::Greater) => self.x.0 = rule.value,
            (Part::M, Operation::Lesser) => self.m.1 = rule.value,
            (Part::M, Operation::Greater) => self.m.0 = rule.value,
            (Part::A, Operation::Lesser) => self.a.1 = rule.value,
            (Part::A, Operation::Greater) => self.a.0 = rule.value,
            (Part::S, Operation::Lesser) => self.s.1 = rule.value,
            (Part::S, Operation::Greater) => self.s.0 = rule.value,
        }
    }

    fn update_second_boundary(&mut self, rule: &Rule) {
        match (&rule.part, &rule.operation) {
            (Part::X, Operation::Lesser) => self.x.0 = rule.value - 1,
            (Part::X, Operation::Greater) => self.x.1 = rule.value + 1,
            (Part::M, Operation::Lesser) => self.m.0 = rule.value - 1,
            (Part::M, Operation::Greater) => self.m.1 = rule.value + 1,
            (Part::A, Operation::Lesser) => self.a.0 = rule.value - 1,
            (Part::A, Operation::Greater) => self.a.1 = rule.value + 1,
            (Part::S, Operation::Lesser) => self.s.0 = rule.value - 1,
            (Part::S, Operation::Greater) => self.s.1 = rule.value + 1,
        }
    }

    fn divide(
        &self,
        workflows: &HashMap<String, Workflow>,
        name: &str,
        index: usize,
    ) -> Vec<MultiRule> {
        let workflow = workflows.get(name).unwrap();
        let rule = &workflow.rules[index];

        let mut sub_vec: Vec<MultiRule> = Vec::new();

        if self.check_valid(rule) {
            let mut new_rule_1 = self.clone();
            new_rule_1.update_first_boundary(rule);
            match &rule.result {
                WorkflowResult::Accepted => {
                    new_rule_1.result = WorkflowResult::Accepted;
                    sub_vec.push(new_rule_1);
                }
                WorkflowResult::Rejected => {
                    new_rule_1.result = WorkflowResult::Rejected;
                    sub_vec.push(new_rule_1);
                }
                WorkflowResult::Workflow(name) => {
                    sub_vec.append(&mut new_rule_1.divide(workflows, name, 0));
                }
                WorkflowResult::Pass => unreachable!("The result shouldn't be Pass"),
            }

            let mut new_rule_2 = self.clone();
            new_rule_2.update_second_boundary(rule);
            if index + 1 < workflow.rules.len() {
                sub_vec.append(&mut new_rule_2.divide(workflows, name, index + 1));
            } else {
                match &workflow.result {
                    WorkflowResult::Accepted => {
                        new_rule_2.result = WorkflowResult::Accepted;
                        sub_vec.push(new_rule_2);
                    }
                    WorkflowResult::Rejected => {
                        new_rule_2.result = WorkflowResult::Rejected;
                        sub_vec.push(new_rule_2);
                    }
                    WorkflowResult::Workflow(name) => {
                        sub_vec.append(&mut new_rule_2.divide(workflows, name, 0));
                    }
                    WorkflowResult::Pass => unreachable!("The result shouldn't be Pass"),
                }
            }
        } else {
            sub_vec.push(self.clone());
        }

        sub_vec
    }

    fn count_possibilities(&self) -> usize {
        let possible_x = self.x.1 - self.x.0 - 1;
        let possible_m = self.m.1 - self.m.0 - 1;
        let possible_a = self.a.1 - self.a.0 - 1;
        let possible_s = self.s.1 - self.s.0 - 1;

        possible_x * possible_m * possible_a * possible_s
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    result: WorkflowResult,
}

impl Workflow {
    fn new() -> Self {
        Self {
            rules: Vec::new(),
            result: WorkflowResult::Rejected,
        }
    }

    fn includes(&self, rating: &Rating) -> WorkflowResult {
        for rule in &self.rules {
            match rule.includes(rating) {
                WorkflowResult::Pass => (),
                result => return result,
            }
        }
        self.result.clone()
    }
}

#[derive(Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn new() -> Self {
        Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
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
            break;
        }
        let (name, rest) = line.split_once('{').unwrap();
        let mut workflow = Workflow::new();
        let rules = &rest[..rest.len() - 1];
        for rule in rules.split(',') {
            if rule.contains(':') {
                workflow.rules.push(parse_rule(rule));
            } else {
                workflow.result = WorkflowResult::new(rule);
            }
        }
        workflows.insert(name.to_string(), workflow);
    }

    let mut ratings: Vec<Rating> = Vec::new();

    for line in part_lines.lines() {
        let mut rating = line.replace('{', "");
        rating = rating.replace('}', "");
        ratings.push(parse_rating(&rating));
    }

    (workflows, ratings)
}

fn parse_rule(rule: &str) -> Rule {
    let (condition, result) = rule.split_once(':').unwrap();

    if condition.contains('<') {
        let (part, value) = condition.split_once('<').unwrap();
        Rule::new(part, value, Operation::Lesser, result)
    } else {
        let (part, value) = condition.split_once('>').unwrap();
        Rule::new(part, value, Operation::Greater, result)
    }
}

fn parse_rating(input: &str) -> Rating {
    let mut rating = Rating::new();

    for string in input.split(',') {
        let (part, value) = string.split_once('=').unwrap();
        match part {
            "x" => rating.x = value.parse::<usize>().unwrap(),
            "m" => rating.m = value.parse::<usize>().unwrap(),
            "a" => rating.a = value.parse::<usize>().unwrap(),
            "s" => rating.s = value.parse::<usize>().unwrap(),
            _ => unreachable!("Unexpected part {part}"),
        }
    }

    rating
}
