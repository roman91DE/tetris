use arrayvec::ArrayVec;

use crate::point::Point;

enum BlockShape {
    Square,
    Line,
    T,
    L,
    LRev,
    Z,
    ZRev,
}

struct Block {
    shape: BlockShape,
    num_rotations: usize,
    coordinates: ArrayVec<Point, 4>
}



impl Block {
    
}