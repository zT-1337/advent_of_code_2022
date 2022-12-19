use crate::util::load_lines_of_file;

struct HeightMap {
    heights: Vec<u8>,
    width: usize,
    start_index: usize,
    end_index: usize,
}

impl HeightMap {
    fn parse_height_map(lines: &Vec<String>) -> Self {
        if lines.len() == 0 || lines[0].len() == 0 {
            panic!("Empty Map");
        }

        let width = lines[0].len();
        let height = lines.len();
        let mut heights = Vec::with_capacity(width * height);

        let mut start_index = 0;
        let mut end_index = 0;

        for line in lines.iter() {
            for height_marker in line.chars() {
                heights.push(HeightMap::parse_height_marker(height_marker));

                if height_marker == 'S' {
                    start_index = heights.len() - 1;
                }

                if height_marker == 'E' {
                    end_index = heights.len() - 1;
                }
            }
        }

        Self {
            width,
            heights,
            start_index,
            end_index,
        }
    }

    fn parse_height_marker(height_marker: char) -> u8 {
        match height_marker {
            'S' => 0,
            'E' => 25,
            _ => height_marker as u8 - 97,
        }
    }

    fn find_all_possible_starting_positions(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for (index, height) in self.heights.iter().enumerate() {
            if *height == 0 {
                result.push(index);
            }
        }

        result
    }

    fn find_shortest_path(&self) -> Option<Vec<usize>> {
        let nodes = DijkstraNode::build_dijkstra_tree_for_start_and_end(self);
        DijkstraNode::try_to_get_path_from_start_to_end(&nodes, self.start_index, self.end_index)
    }

    fn find_neighbours(&self, current_position: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(4);

        // has top neighbour
        if current_position >= self.width {
            let top_position = current_position - self.width;
            if self.is_climable(current_position, top_position) {
                result.push(top_position);
            }
        }

        // has bottom neighbour
        if current_position < self.heights.len() - self.width {
            let bottom_position = current_position + self.width;
            if self.is_climable(current_position, bottom_position) {
                result.push(bottom_position);
            }
        }

        // has left neighbour
        if current_position % self.width != 0 {
            let left_position = current_position - 1;
            if self.is_climable(current_position, left_position) {
                result.push(left_position);
            }
        }

        // has right neighbour
        if (current_position + 1) % self.width != 0 {
            let right_position = current_position + 1;
            if self.is_climable(current_position, right_position) {
                result.push(right_position);
            }
        }

        result
    }

    fn is_climable(&self, current_position: usize, target_position: usize) -> bool {
        let current_height = self.heights[current_position];
        let target_height = self.heights[target_position];

        current_height > target_height || target_height - current_height < 2
    }
}

#[derive(Debug)]
struct DijkstraNode {
    position: usize,
    cost: u64,
    previous: Option<usize>,
}

impl DijkstraNode {
    fn try_to_get_path_from_start_to_end(
        nodes: &Vec<DijkstraNode>,
        start: usize,
        end: usize,
    ) -> Option<Vec<usize>> {
        if nodes[end].previous.is_none() {
            return None;
        }

        let mut result = Vec::new();

        let mut current_position = end;
        while current_position != start {
            result.push(current_position);
            current_position = nodes[current_position].previous.unwrap();
        }
        result.reverse();

        Some(result)
    }

    fn build_dijkstra_tree_for_start_and_end(height_map: &HeightMap) -> Vec<DijkstraNode> {
        let mut nodes = Self::from_height_map(height_map);
        let mut unvisited_node_positions =
            Self::init_unvisited_node_positions(height_map.heights.len());

        loop {
            let current_node =
                match Self::find_cheapest_unvisited_node(&nodes, &mut unvisited_node_positions) {
                    Some(position) => &nodes[position],
                    None => break,
                };

            //Only happens if no node exists anymore that is connected somehow with the start
            if current_node.cost == u64::MAX {
                break;
            }

            let current_node_position = current_node.position;
            //every neighbour is always one step away
            let cost_to_neighbours = current_node.cost + 1;

            let neighbour_positions = height_map.find_neighbours(current_node_position);
            for neighbour_position in neighbour_positions {
                let neighbour = &mut nodes[neighbour_position];
                if cost_to_neighbours < neighbour.cost {
                    neighbour.cost = cost_to_neighbours;
                    neighbour.previous = Some(current_node_position);
                }
            }
        }

        nodes
    }

    fn from_height_map(height_map: &HeightMap) -> Vec<Self> {
        let mut result = Vec::with_capacity(height_map.heights.len());
        for i in 0..height_map.heights.len() {
            result.push(Self {
                position: i,
                cost: u64::MAX,
                previous: None,
            });
        }
        result[height_map.start_index].cost = 0;

        result
    }

    fn init_unvisited_node_positions(node_count: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(node_count);

        for i in 0..node_count {
            result.push(i);
        }

        result
    }

    fn find_cheapest_unvisited_node(
        nodes: &Vec<DijkstraNode>,
        unvisited_nodes_positions: &mut Vec<usize>,
    ) -> Option<usize> {
        let mut min_cost = u64::MAX;
        let mut current_cheapest_node_position_in_q = 0;

        if unvisited_nodes_positions.len() == 0 {
            return None;
        }

        for (index, node_position) in unvisited_nodes_positions.iter().enumerate() {
            let node = &nodes[*node_position];
            if node.cost <= min_cost {
                min_cost = node.cost;
                current_cheapest_node_position_in_q = index;
            }
        }

        Some(unvisited_nodes_positions.remove(current_cheapest_node_position_in_q))
    }
}

pub fn day_12_star_1_and_2() {
    let mut lines = load_lines_of_file("/home/zt/Workspace/advent_of_code/src/day12.input");
    lines.pop();

    let mut height_map = HeightMap::parse_height_map(&lines);

    println!(
        "Result of Advent of Code Day 12, Star 1: {}",
        height_map.find_shortest_path().unwrap().len()
    );

    let starting_positions = height_map.find_all_possible_starting_positions();
    let mut routes: Vec<Vec<usize>> = Vec::new();

    for starting_position in starting_positions.iter() {
        height_map.start_index = *starting_position;
        if let Some(route) = height_map.find_shortest_path() {
            routes.push(route);
        }
    }

    println!(
        "Result of Advent of Code Day 12, Star 2: {}",
        routes.iter().map(|route| route.len()).min().unwrap(),
    );
}
