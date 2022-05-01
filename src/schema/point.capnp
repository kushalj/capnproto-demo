@0xb068ff5fb1c4f77e;

using Rust = import "rust.capnp";
$Rust.parentModule("server");

struct Point {
    x @0 :Float32;
    y @1 :Float32;
}

interface PointTracker {
    addPoint @0 (p :Point) -> (totalPoints :UInt64);
}

