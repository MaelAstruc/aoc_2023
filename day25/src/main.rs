use std::collections::HashMap;

fn main() {
    part_1(
        "jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr",
    );
}

#[derive(Debug)]
struct Graph {
    components: HashMap<String, Component>,
}

impl Graph {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    fn push(&mut self, component: Component) {
        if !self.components.contains_key(&component.name) {
            self.components
                .insert(component.name.clone(), component.clone());
        }

        for other in component.connections {
            self.add_connection(&component.name, other);
        }
    }

    fn add_connection(&mut self, name: &String, to: String) {
        match self.components.get_mut(&to) {
            Some(compo) => if !compo.connections.contains(name) {
                compo.connections.push(name.clone())
            }
            None => self.push(Component::new(to.clone(), vec![name.clone()])),
        }
    }

    fn reset_groups(&mut self) {
        for (_name, compo) in &mut self.components {
           compo.group = None; 
        }
    }
    
    fn check_groups(&mut self) {
        let mut index: usize = 0;
        for (name, _compo) in &self.components {
           self.update_group(name, index);
           index += 1;
        }
    }
    
    fn update_group(&mut self, name: &String, index: usize) {
        let compo = self.components.get_mut(name).unwrap(); 
        match compo.group {
            Some(_) => (),
            None => {
                compo.group = Some(index);
                for other in &compo.connections {
                    self.update_group(other, index);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Component {
    name: String,
    connections: Vec<String>,
    group: Option<usize>
}

impl Component {
    fn new(name: String, connections: Vec<String>) -> Self {
        Self { name, connections, group: None }
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        let (first, others) = line.split_once(": ").unwrap();
        let name = first.trim().to_string();
        let connections: Vec<String> = others.split(' ').map(|x| x.to_string()).collect();
        let compo = Component::new(name, connections);
        graph.push(compo);
    }

    return graph;
}

fn part_1(input: &str) {
    let graph = parse_input(input);



    println!("{:#?}", graph);
}
