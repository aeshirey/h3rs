use crate::GeoCoord;

/// Maximum number of cell boundary vertices; worst case is pentagon: 5 original verts + 5 edge crossings
const MAX_CELL_BNDRY_VERTS: usize = 10;

/// cell boundary in latitude/longitude
pub struct GeoBoundary {
    /// number of vertices
    pub numVerts: usize,

    /// vertices in ccw order
    pub verts: [GeoCoord; MAX_CELL_BNDRY_VERTS],
}

/// similar to GeoBoundary, but requires more alloc work
pub(crate) struct Geofence {
    pub verts: Vec<GeoCoord>,
}

/// Simplified core of GeoJSON Polygon coordinates definition
pub(crate) struct GeoPolygon {
    /// exterior boundary of the polygon
    pub geofence: Geofence,

    /// interior boundaries (holes) in the polygon
    pub holes: Vec<Geofence>,
}

/// Simplified core of GeoJSON MultiPolygon coordinates definition
pub(crate) struct GeoMultiPolygon {
    pub polygons: Vec<GeoPolygon>,
}
