/// `Node` is a structure that represents a node in TMM.
///
/// # Fields
///
/// * `id` - A node ID number.  
/// * `ti` - A floating point number representing the initial temperature of the node.
/// * `cp` - A floating point number representing the capacitance of the node.
/// * `ntype` - The type of the node in the data structure. This parameter determines the behavior and properties of the node.
pub struct Node {
    id: i32,
    ti: f64,
    cp: f64,
    ntype: NodeType
}

/// `NodeType` is an enumeration representing different types of nodes in a data structure.
/// The type of node is determined by the `id` and `cp` values.
///
/// Variants:
/// * `Diffusion`: Represents a diffusion node. A diffusion node shall have `id > 0` and `cp > 0`.
/// * `Arithmetic`: Represents a arithmetic node. An arithmetic node shall have `id > 0` and `cp < 0`.
/// * `Boundary`: Represents a boundary node. A boundary node shall have `id < 0` and `cp = 0`.
pub enum NodeType {
    Diffusion,
    Arithmetic,
    Boundary,
}
