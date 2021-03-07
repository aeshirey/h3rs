
/// A single node in a vertex graph, part of a linked list
struct VertexNode {
    from: GeoCoord,
    to: GeoCoord,
    next: Box<VertexNode>,
};

/// A data structure to store a graph of vertices
struct VertexGraph {
    //VertexNode** buckets;
    numBuckets:i32,
    size:i32,
    res:i32,
}

