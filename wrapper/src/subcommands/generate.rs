use std::collections::HashSet;

use graph_generator;
use ayudame_core_rs::ayu_events::*;

use super::generate_mem_address_from_id;

pub(crate) fn run_generate_graph() {
    let num_nodes = 22;
    let edges_per_node = 2;
    let graph = graph_generator::GraphLayout::new_from_num_nodes(num_nodes, edges_per_node);
    let edges = graph.build_edges();
    let mut tasks = HashSet::new();
    initialize_with_frontend();
    // send graph to ayudame    

    for (predecessor, successor) in edges.into_iter().map(|(p, s)| (p as u64, s as u64)) {
        if tasks.insert(predecessor) {
            ayu_event_addtask(predecessor, predecessor % 15, 0, 0);
            // ayu_event_runtask(predecessor);
        }

        tasks.insert(successor);
        ayu_event_addtask(successor, successor % 15, 0, 0);
        ayu_event_adddependency(successor, predecessor, generate_mem_address_from_id(successor), generate_mem_address_from_id(predecessor));
        // ayu_event_runtask(successor);
    }
}

fn initialize_with_frontend() {
    ayu_event_preinit(0);
    ayu_event_init(0);
}