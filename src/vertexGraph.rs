use crate::h3index::Resolution;
use crate::GeoCoord;

/// A single node in a vertex graph, part of a linked list
struct VertexNode {
    from: GeoCoord,
    to: GeoCoord,
    next: Box<VertexNode>,
}

/// A data structure to store a graph of vertices
struct VertexGraph {
    //VertexNode** buckets;
    buckets: Option<Vec<VertexNode>>,
    numBuckets: i32,
    size: i32,
    res: Resolution,
}

impl VertexGraph {
    /**
     * Initialize a new VertexGraph
     * @param graph       Graph to initialize
     * @param  numBuckets Number of buckets to include in the graph
     * @param  res        Resolution of the hexagons whose vertices we're storing
     */
    fn initVertexGraph(numBuckets: i32, res: Resolution) -> Self {
        let buckets = if numBuckets > 0 {
            Some(Vec::with_capacity(numBuckets as usize))
        } else {
            None
        };

        Self {
            numBuckets,
            res,
            size: 0,
            buckets,
        }
    }
}
