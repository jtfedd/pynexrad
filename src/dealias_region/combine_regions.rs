use crate::dealias_region::edge_tracker::EdgeTracker;
use crate::dealias_region::region_tracker::RegionTracker;

pub(crate) fn combine_regions(
    region_tracker: &mut RegionTracker,
    edge_tracker: &mut EdgeTracker,
) -> bool {
    // Returns true when done

    // Edge parameters from edge with largest weight
    let (status, extra) = edge_tracker.pop_edge();
    if status {
        return true;
    }

    let (node1, node2, _weight, diff, edge_number) = extra;
    let mut rdiff = diff.round() as i32;

    let node1_size = region_tracker.get_node_size(node1);
    let node2_size = region_tracker.get_node_size(node2);

    let (base_node, merge_node) = {
        if node1_size > node2_size {
            (node1, node2)
        } else {
            rdiff = -rdiff;
            (node2, node1)
        }
    };

    if rdiff != 0 {
        region_tracker.unwrap_node(merge_node, rdiff);
        edge_tracker.unwrap_node(merge_node, rdiff);
    }

    region_tracker.merge_nodes(base_node, merge_node);
    edge_tracker.merge_nodes(base_node, merge_node, edge_number);

    return false;
}
