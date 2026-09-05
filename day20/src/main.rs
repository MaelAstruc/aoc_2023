use std::collections::{HashMap, VecDeque};

fn main() {
    part_1(
        "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a",
    );

    part_1(
        "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"
    );

    //part_1(&std::fs::read_to_string("input.txt").unwrap());
    
    part_2("broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> rx");

    //part_2(&std::fs::read_to_string("input.txt").unwrap());
}

/*
Determine the state needed to send one low pulse to the last
? Which pulse are sent during a turn
- A pulse sent depends on the current state and the pulse received from the sources
- The current state depends on the pulse previously received from the sources
=> 2 conditions
Determine the number of steps needed to attain such a state
- 

rx
- receive: low
- need: low from 


broadcast:  1 (low)
a:          2 (off, on) -> (high low)
inv:        2 (high, low) -> (low high)
con:        ((low low) (high))
b:          4 (off, on, on, off) -> (high X low X)
*/

/*
Last: one low pulse
Flip flop
    - low: low % 2 == 0
    - high: low % 2 == 1
Conjunction
    - low: all hight
    - high: any low
*/

/*
broadcast
a low
b low

rx <- con low
con low <- a high & b high
a high (low % 2 == 1) <- broadcast
b high <- inv (low % 2 == 1)
inv low * 2 <- a high * 4
a high * 4 <- broadcast * 8 
*/

fn part_1(input: &str) {
    let mut graph = parse_input(input);

    for _ in 0..1000 {
        graph.pulse();
    }

    let total = graph.count_low * graph.count_high;
    
    println!("low: {}; high: {} => {total}", graph.count_low, graph.count_high);
}

fn part_2(input: &str) {
    let mut graph = parse_input(input);

    for i in 0..4 {
        println!("{i}");
        graph.pulse();
        
        println!("{:#?}", graph.modules.get(1));
    }

    /*loop {
        let module = graph.modules.get(index).unwrap();

        match module {
            Module::FlipFlop(_) => todo!(),
            Module::Conjunction(_) => todo!(),
            Module::Broadcast(_) => todo!(),
            Module::Empty(_) => todo!(),
        }
    }*/
}

/*
fn part_2(input: &str) {
    let mut graph = parse_input(input);

    let mut count: usize = 0;

    let last = graph.modules[graph.end.unwrap()].clone();

    let previous  = match &last {
        Module::Empty(module) => module.source,
        _ => unreachable!()
    };

    let mut map: HashMap<usize, usize> = HashMap::new();

    println!("{last:?}");
    println!("{:?}", &graph.modules[previous]);
    
    loop {
        if count % 1_000_000 == 0 {
            println!("{count}");
        }
        count += 1;

        graph.count_low_end = 0;

        graph.pulse();

        let previous_module = &graph.modules[previous];
        
        if let Module::Conjunction(module) = previous_module {
            let mut all_included = true;
            for (source, _) in &module.sources {
                if module.sources.get(source).unwrap() == &Pulse::High {
                    println!("{source}, {count}");
                    map.insert(*source, count);
                }
                if !map.contains_key(source) {
                    all_included = false;
                }
            }
            if all_included {
                break;
            }
        }

    }

    println!("=>{count}");
}
*/

#[derive(Debug)]
struct Graph {
    start: usize,
    end: Option<usize>,
    modules: Vec<Module>,
    count_low: usize,
    count_high: usize,
    count_low_end: usize,
}

impl Graph {
    fn new(start: usize, end: Option<usize>, size: usize) -> Self {
        Self {
            start,
            end,
            modules: vec![Module::Broadcast(Broadcast::new()); size],
            count_low: 0,
            count_high: 0,
            count_low_end: 0,
        }
    }

    fn add_module_at(&mut self, at: usize, module: Module) {
        self.modules[at] = module;
    }

    fn link(&mut self, source: usize, target: usize) {
        self.modules[source].add_target(target);
        self.modules[target].add_source(source);
    }

    fn pulse(&mut self) {
        let mut pulsed: VecDeque<(Pulse, usize, usize)> = VecDeque::new();
        pulsed.push_back((Pulse::Low, self.start, self.start));

        while let Some((pulse, from, to)) = pulsed.pop_front() {
            //println!("{pulse:_>4?} / {from} => {to}");
            match pulse {
                Pulse::Low => self.count_low += 1,
                Pulse::High => self.count_high += 1,
            }

            if Some(to) == self.end {
                match pulse {
                    Pulse::Low => self.count_low_end += 1,
                    Pulse::High => (),
                }
            }

            let new_pulses = self.modules[to].pulse(pulse, from);
            for (pulse, destination) in new_pulses {
                //println!("\t{:?}", (pulse, to, destination));
                pulsed.push_back((pulse, to, destination));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
    Empty(Empty),
}

impl Module {
    fn add_target(&mut self, target: usize) {
        match self {
            Module::FlipFlop(module) => module.add_target(target),
            Module::Conjunction(module) => module.add_target(target),
            Module::Broadcast(module) => module.add_target(target),
            Module::Empty(_) => (),
        }
    }

    fn add_source(&mut self, source: usize) {
        match self {
            Module::FlipFlop(module) => module.add_source(source),
            Module::Conjunction(module) => module.add_source(source),
            Module::Broadcast(module) => (),
            Module::Empty(module) => module.add_source(source),
        }
    }

    fn pulse(&mut self, pulse: Pulse, from: usize) -> Vec<(Pulse, usize)> {
        match self {
            Module::FlipFlop(module) => module.pulse(pulse),
            Module::Conjunction(module) => module.pulse(pulse, from),
            Module::Broadcast(module) => module.pulse(pulse),
            Module::Empty(_) => {
                println!("\t{from} => {pulse:?}");
                Vec::new()
            },
        }
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    state: bool,
    destinations: Vec<usize>,
    sources: HashMap<usize, Pulse>,
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            state: false,
            destinations: Vec::new(),
            sources: HashMap::new(),
        }
    }

    fn add_target(&mut self, target: usize) {
        self.destinations.push(target)
    }

    fn add_source(&mut self, source: usize) {
        self.sources.insert(source, Pulse::Low);
    }

    fn pulse(&mut self, pulse: Pulse) -> Vec<(Pulse, usize)> {
        if pulse == Pulse::Low {
            self.state = !self.state;
        }
        match (pulse, self.state) {
            (Pulse::Low, true) => self
                .destinations
                .iter()
                .map(|destination| (Pulse::High, *destination))
                .collect(),
            (Pulse::Low, false) => self
                .destinations
                .iter()
                .map(|destination| (Pulse::Low, *destination))
                .collect(),
            (Pulse::High, _) => Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    destinations: Vec<usize>,
    sources: HashMap<usize, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            destinations: Vec::new(),
            sources: HashMap::new(),
        }
    }

    fn add_target(&mut self, target: usize) {
        self.destinations.push(target)
    }

    fn add_source(&mut self, source: usize) {
        self.sources.insert(source, Pulse::Low);
    }

    fn pulse(&mut self, pulse: Pulse, from: usize) -> Vec<(Pulse, usize)> {
        self.sources.insert(from, pulse);
        if self.is_high() {
            self.destinations
                .iter()
                .map(|destination| (Pulse::Low, *destination))
                .collect()
        } else {
            self.destinations
                .iter()
                .map(|destination| (Pulse::High, *destination))
                .collect()
        }
    }

    fn is_high(&self) -> bool {
        for (_, pulse) in &self.sources {
            if pulse == &Pulse::Low {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Broadcast {
    destinations: Vec<usize>,
}

impl Broadcast {
    fn new() -> Self {
        Self {
            destinations: Vec::new(),
        }
    }

    fn add_target(&mut self, target: usize) {
        self.destinations.push(target)
    }

    fn pulse(&mut self, _pulse: Pulse) -> Vec<(Pulse, usize)> {
        self.destinations
            .iter()
            .map(|destination| (Pulse::Low, *destination))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Empty {
    source: usize,
}

impl Empty {
    fn new() -> Self {
        Self {
            source: 0,
        }
    }

    fn add_source(&mut self, source: usize) {
        self.source = source;
    }
}

fn parse_input(input: &str) -> Graph {
    let mut modules: HashMap<String, (usize, Module, Vec<String>)> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let (module, destinations) = line.split_once(" -> ").unwrap();

        let destinations: Vec<String> = destinations.split(", ").map(|s| s.to_string()).collect();

        if module.trim().starts_with('%') {
            let name = module.trim().replace('%', "");
            modules.insert(name, (i, Module::FlipFlop(FlipFlop::new()), destinations));
        } else if module.trim().starts_with('&') {
            let name = module.trim().replace('&', "");
            modules.insert(
                name,
                (i, Module::Conjunction(Conjunction::new()), destinations),
            );
        } else if module.trim() == "broadcaster" {
            let name = "broadcaster".to_string();
            modules.insert(name, (i, Module::Broadcast(Broadcast::new()), destinations));
        } else {
            unreachable!("Unexpected module {}", module.trim());
        };
    }

    // Complete unknown destinations
    let mut all_destinations: Vec<String> = Vec::new();
    for (_, (_, _, destinations)) in &mut modules {
        for destination in destinations {
            if !all_destinations.contains(&destination) {
                all_destinations.push(destination.to_string())
            }
        }
    }

    let mut end = None;

    for destination in all_destinations {
        match modules.get(&destination) {
            Some(_) => (),
            None => {
                if destination == "rx" {
                    end = Some(modules.len());
                }
                modules.insert(destination.to_string(), (modules.len(), Module::Empty(Empty::new()), Vec::new()));
            },
        };
    }

    let (start, _, _) = modules.get("broadcaster").unwrap();

    let mut graph = Graph::new(*start, end, modules.len());

    // Populate the graph
    for (_, module) in &modules {
        graph.add_module_at(module.0, module.1.clone());
    }

    for (_, (source, _, destinations)) in &modules {
        for name in destinations {
            let (target, _, _) = modules.get(name).unwrap();
            graph.link(*source, *target);
        }
    }

    graph
}
