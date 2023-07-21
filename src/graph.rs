use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub color: Color,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub source_id: String,
    pub dest_id: String,
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

fn update_step_length(step_length: f64, energy: f64, old_energy: f64, progress: &mut i64) -> f64 {
    if energy < old_energy {
        *progress = *progress + 1i64;
        if *progress >= 5 {
            *progress = 0i64;
        }
        step_length / 0.9
    } else {
        *progress = 0;
        step_length * 0.9
    }
}

// Based on http://yifanhu.net/PUB/graph_draw_small.pdf
pub fn layout(
    mut graph: Graph,
    optimal_spring_length: f64,
    force_strength: f64,
    initial_step_length: f64,
) -> Graph {
    let mut converged = false;
    let mut energy = std::f64::INFINITY;
    let mut step_length: f64 = initial_step_length;
    let mut progress: i64 = 0;
    let mut node_map = HashMap::new();
    let mut adjacency_map: HashMap<String, Vec<&Node>> = HashMap::new();

    for i in 0..graph.nodes.len() {
        node_map.insert(graph.nodes[i].id.clone(), &graph.nodes[i]);
        adjacency_map.insert(graph.nodes[i].id.clone(), Vec::new());
    }

    for edge in graph.edges.iter() {
        let dest_node = node_map.get(&edge.dest_id).unwrap();

        adjacency_map
            .get_mut(&edge.source_id)
            .unwrap()
            .push(*dest_node);
    }

    let mut new_graph = graph.clone();

    while (!converged) {
        // Store the current positions of all the vertices
        let old_positions = graph.nodes.clone();
        let old_energy = energy;
        energy = 0.0;

        for node in graph.nodes.iter() {
            let mut force_x: f64 = 0.0;
            let mut force_y: f64 = 0.0;

            for dest_node in adjacency_map.get(&node.id).unwrap().iter() {
                let dx = dest_node.x - node.x;
                let dy = dest_node.y - node.y;
                let distance_squared = dx * dx + dy * dy;
                let distance = distance_squared.sqrt();
                let attractive_force = distance_squared / optimal_spring_length;
                force_x = force_x + attractive_force * dx / distance;
                force_y = force_y + attractive_force * dy / distance;

                println!("Attractive force {:?}", attractive_force);
            }

            for other_node in graph.nodes.iter().filter(|n| n.id != node.id) {
                let dx = other_node.x - node.x;
                let dy = other_node.y - node.y;
                let distance_squared = dx * dx + dy * dy;
                let distance = distance_squared.sqrt();
                let repulsive_force =
                    -force_strength * optimal_spring_length * optimal_spring_length
                        / distance_squared;
                force_x = force_x + repulsive_force * dx / distance;
                force_y = force_y + repulsive_force * dy / distance;
                println!(
                    "Repulsive force from {:?} to {:?}: {:?}",
                    node.id, other_node.id, repulsive_force
                );
            }

            let force_length_squared = force_x * force_x + force_y * force_y;
            let force_length = force_length_squared.sqrt();

            let mut shift_x: f64 = 0.0;
            let mut shift_y: f64 = 0.0;
            if force_length > 0.0 {
                shift_x = step_length * force_x / force_length;
                shift_y = step_length * force_y / force_length;
            }

            println!("Node {:?} shift: {:?}, {:?}", node.id, force_x, force_y);

            let new_node = new_graph
                .nodes
                .iter_mut()
                .find(|n| n.id == node.id)
                .unwrap();
            new_node.x = new_node.x + shift_x;
            new_node.y = new_node.y + shift_y;

            energy = energy + force_length_squared;
        }

        step_length = update_step_length(step_length, energy, old_energy, &mut progress);

        new_graph = new_graph.clone();

        converged = true;
        for old_node in old_positions.iter() {
            let new_node = new_graph
                .nodes
                .iter()
                .find(|n| n.id == old_node.id)
                .unwrap();
            let dx = new_node.x - old_node.x;
            let dy = new_node.y - old_node.y;
            let distance_squared = dx * dx + dy * dy;
            let distance = distance_squared.sqrt();
            if distance > optimal_spring_length {
                converged = false;
                break;
            }
        }

        println!("New graph: {:?}", new_graph);
    }
    println!("Energy is {}", energy);
    new_graph
}
