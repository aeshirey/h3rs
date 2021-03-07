
/// A polygon node in a linked geo structure, part of a linked list.
struct LinkedGeoPolygon {
    LinkedGeoLoop *first;
    LinkedGeoLoop *last;
    next : Option<Box<LinkedGeoPolygon>>
};
