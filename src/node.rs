/// `Node` is a structure that represents a node in TMM.
///
/// # Fields
///
/// * `id` - A node ID number.  
/// * `ti` - A floating point number representing the initial temperature of the node.
/// * `cp` - A floating point number representing the capacitance of the node.
pub struct Node {
    id: i32,
    ti: f64,
    cp: f64,
    ntype: NodeType
}

pub enum NodeType {
    Diffusion,
    Arithmetic,
    Boundary,
}

// id > 0 and cp > 0, then "diffusion node"
// id > 0 and cp < 0, then "arithmetic node"
// id < 0 and cp = 0, then "boundary node"
