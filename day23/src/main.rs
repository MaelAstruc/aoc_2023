use std::collections::HashMap;

fn main() {
    assert_eq!(
        part_1(
            "#.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#",
        ),
        94
    );

    assert_eq!(
        part_2(
            "#.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#"
        ),
        154
    );

    part_2(&std::fs::read_to_string("input.txt").unwrap());
}

/* Assumptions
- A slope is always in a straight path
*/

fn part_1(input: &str) -> usize {
    // Scan the map
    let map = parse_input(input);

    // Summarize the paths with nodes and links
    let graph = Graph::from(&map);

    let node_id = graph.start;
    let start_path = Path::new(node_id);
    let start_neighbors = graph.get_neighbors(node_id);

    let mut edges_to_check: Vec<(EdgeId, Path)> = Vec::new();
    let mut complete_paths: Vec<Path> = Vec::new();

    for neighbor in start_neighbors {
        edges_to_check.push((neighbor, start_path.clone()));
    }

    while let Some((edge_id, path)) = edges_to_check.pop() {
        let link = graph.get_link(edge_id).unwrap();

        let last_node = path.nodes.last().unwrap();

        let new_node_id = match (link.direction, link.node_a == *last_node) {
            (Direction::Impossible, _) => continue,
            (Direction::Forward, false) => continue,
            (Direction::Backward, true) => continue,
            (_, true) => link.node_b,
            (_, false) => link.node_a,
        };

        if path.nodes.contains(&new_node_id) {
            continue;
        }

        let new_path = Path::new_extension(path, new_node_id, link.length);

        if new_node_id == graph.end {
            complete_paths.push(new_path);
            continue;
        }

        let new_node = graph.get_node(new_node_id).unwrap();

        for neighbor in &new_node.neighbors {
            if *neighbor != edge_id {
                edges_to_check.push((*neighbor, new_path.clone()));
            }
        }
    }

    let longest_hike = complete_paths.iter().map(|path| path.length).max().unwrap();

    println!("{longest_hike}");

    longest_hike
}

fn part_2(input: &str) -> usize {
    // Scan the map
    let map = parse_input(input);

    // Summarize the paths with nodes and links
    let graph = Graph::from(&map);

    let node_id = graph.start;
    let start_path = Path::new(node_id);
    let start_neighbors = graph.get_neighbors(node_id);

    let mut edges_to_check: Vec<(EdgeId, Path)> = Vec::new();
    let mut complete_paths: Vec<Path> = Vec::new();

    for neighbor in start_neighbors {
        edges_to_check.push((neighbor, start_path.clone()));
    }

    while let Some((edge_id, path)) = edges_to_check.pop() {
        let link = graph.get_link(edge_id).unwrap();

        let last_node = path.nodes.last().unwrap();

        // Only change
        let new_node_id = if link.node_a == *last_node {
            link.node_b
        } else {
            link.node_a
        };

        if path.nodes.contains(&new_node_id) {
            continue;
        }

        let new_path = Path::new_extension(path, new_node_id, link.length);

        if new_node_id == graph.end {
            complete_paths.push(new_path);
            continue;
        }

        let new_node = graph.get_node(new_node_id).unwrap();

        for neighbor in &new_node.neighbors {
            if *neighbor != edge_id {
                edges_to_check.push((*neighbor, new_path.clone()));
            }
        }
    }

    let longest_hike = complete_paths.iter().map(|path| path.length).max().unwrap();

    println!("{longest_hike}");

    longest_hike
}

#[derive(Debug, Clone)]
struct Path {
    nodes: Vec<NodeId>,
    length: usize,
}

impl Path {
    fn new(first_node_id: NodeId) -> Self {
        Self {
            nodes: vec![first_node_id],
            length: 0,
        }
    }

    fn new_extension(path: Path, node_id: usize, length: usize) -> Self {
        let mut new_path = path;
        new_path.nodes.push(node_id);
        new_path.length += length;
        new_path
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    SlopeLeft,
    SlopeRight,
    SlopeUp,
    SlopeDown,
}

impl Tile {
    fn new(character: char) -> Self {
        match character {
            '.' => Tile::Path,
            '<' => Tile::SlopeLeft,
            '>' => Tile::SlopeRight,
            '^' => Tile::SlopeUp,
            'v' => Tile::SlopeDown,
            _ => unreachable!("Unexpected character: {character}"),
        }
    }

    fn new_direction(self, last_position: Position) -> Direction {
        match (self, last_position) {
            (Tile::Path, _) => Direction::All,
            (Tile::SlopeLeft, Position::Left) => Direction::Backward,
            (Tile::SlopeLeft, Position::Right) => Direction::Forward,
            (Tile::SlopeRight, Position::Left) => Direction::Forward,
            (Tile::SlopeRight, Position::Right) => Direction::Backward,
            (Tile::SlopeUp, Position::Up) => Direction::Backward,
            (Tile::SlopeUp, Position::Down) => Direction::Forward,
            (Tile::SlopeDown, Position::Up) => Direction::Forward,
            (Tile::SlopeDown, Position::Down) => Direction::Backward,
            (_, _) => Direction::All,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<usize, Tile>,
    nrow: usize,
    ncol: usize,
}

impl Map {
    fn new(nrow: usize, ncol: usize) -> Self {
        Self {
            tiles: HashMap::new(),
            nrow,
            ncol,
        }
    }

    fn insert(&mut self, id: usize, tile: Tile) {
        self.tiles.insert(id, tile);
    }

    fn contains(&self, id: usize) -> bool {
        self.tiles.contains_key(&id)
    }

    fn get(&self, index: usize) -> &Tile {
        self.tiles.get(&index).unwrap()
    }

    fn get_neighbors(&self, id: usize) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();

        if id > self.ncol && self.contains(id - self.ncol) {
            neighbors.push(id - self.ncol);
        }

        if id < (self.ncol - 1) * self.ncol && self.contains(id + self.ncol) {
            neighbors.push(id + self.ncol);
        }

        if id % self.ncol != 0 && self.contains(id - 1) {
            neighbors.push(id - 1);
        }

        if id % self.ncol != self.ncol - 1 && self.contains(id + 1) {
            neighbors.push(id + 1);
        }

        neighbors
    }
}

type NodeId = usize;

#[derive(Debug)]
struct Node {
    neighbors: Vec<usize>,
}

impl Node {
    fn new(neighbors: Vec<usize>) -> Self {
        Self { neighbors }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward,
    Backward,
    All,
    Impossible,
}

impl Direction {
    fn combine(self, other_direction: Direction) -> Direction {
        match (self, other_direction) {
            (Direction::All, _) => other_direction,
            (_, Direction::All) => self,
            (Direction::Impossible, _) => Direction::Impossible,
            (_, Direction::Impossible) => Direction::Impossible,
            (Direction::Forward, Direction::Forward) => Direction::Forward,
            (Direction::Forward, Direction::Backward) => Direction::Impossible,
            (Direction::Backward, Direction::Forward) => Direction::Impossible,
            (Direction::Backward, Direction::Backward) => Direction::Backward,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Link {
    node_a: NodeId,
    node_b: NodeId,
    length: usize,
    direction: Direction,
}

impl Link {
    fn new(node_a: NodeId, node_b: NodeId, length: usize, direction: Direction) -> Self {
        Self {
            node_a,
            node_b,
            length,
            direction,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Position {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn new(current_index: usize, last_id: usize) -> Self {
        if last_id < current_index {
            if last_id == current_index - 1 {
                Position::Left
            } else {
                Position::Up
            }
        } else if last_id == current_index + 1 {
            Position::Right
        } else {
            Position::Down
        }
    }
}

type EdgeId = usize;
type LinkId = usize;

#[derive(Debug)]
struct Graph {
    start: NodeId,
    end: NodeId,
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, LinkId>,
    links: Vec<Link>,
}

impl Graph {
    fn new(start: NodeId, end: NodeId) -> Self {
        Self {
            start,
            end,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            links: Vec::new(),
        }
    }

    fn add_node(&mut self, index: NodeId, node: Node) {
        self.nodes.insert(index, node);
    }

    fn add_link(&mut self, edge_a: EdgeId, edge_b: EdgeId, link: Link) {
        let link_id = self.links.len();
        self.links.push(link);
        self.edges.insert(edge_a, link_id);
        self.edges.insert(edge_b, link_id);
    }

    fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.edges.contains_key(&edge_id)
    }

    fn get_node(&self, node_id: NodeId) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    fn get_link(&self, edge_id: EdgeId) -> Option<&Link> {
        let link_id = self.edges.get(&edge_id)?;
        self.links.get(*link_id)
    }

    fn get_neighbors(&self, node_id: NodeId) -> Vec<usize> {
        let node = self.get_node(node_id).unwrap();
        node.neighbors.clone()
    }
}

impl From<&Map> for Graph {
    fn from(map: &Map) -> Self {
        let mut graph = Graph::new(1, map.nrow * map.ncol - 2);

        let start_id = 1;
        let start_node = Node::new(map.get_neighbors(start_id));

        graph.add_node(start_id, start_node);

        let mut to_check: Vec<(NodeId, EdgeId)> = vec![(start_id, start_id + map.ncol)];

        while let Some((node_a, first_index)) = to_check.pop() {
            let mut last_index = node_a;
            let mut edge_index = first_index;

            let mut length: usize = 1;
            let mut direction = Direction::All;

            loop {
                let last_position = Position::new(edge_index, last_index);
                let new_tile = map.get(edge_index);

                let new_direction = new_tile.new_direction(last_position);

                direction = direction.combine(new_direction);

                let neighbors = map.get_neighbors(edge_index);

                match neighbors.len() {
                    2 => {
                        length += 1;

                        let new_index = *neighbors.iter().find(|id| **id != last_index).unwrap();
                        last_index = edge_index;
                        edge_index = new_index;
                    }
                    1 | 3 | 4 => {
                        if !graph.contains_edge(first_index) & !graph.contains_edge(last_index) {
                            let link = Link::new(node_a, edge_index, length, direction);
                            graph.add_link(first_index, last_index, link);
                        }

                        for neighbor in &neighbors {
                            if !to_check.contains(&(edge_index, *neighbor))
                                & !graph.contains_edge(*neighbor)
                            {
                                to_check.push((edge_index, *neighbor));
                            }
                        }

                        graph.add_node(edge_index, Node::new(neighbors));
                        break;
                    }
                    x => unreachable!("Unexpected number of neighbors: {x}"),
                }
            }
        }

        graph
    }
}

fn parse_input(input: &str) -> Map {
    let input_clean: Vec<char> = input
        .chars()
        .filter(|c| (c != &' ') & (c != &'\t') & (c != &'\r') & (c != &'\n'))
        .collect();

    let nrow = input.lines().count();
    let ncol = input.lines().next().unwrap().len();

    let mut map: Map = Map::new(nrow, ncol);

    for (id, c) in input_clean.iter().enumerate() {
        if *c == '#' {
            continue;
        }

        let tile = Tile::new(*c);

        map.insert(id, tile);
    }

    map
}
