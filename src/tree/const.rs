
// Initially-allocated block of child nodes for a given Reset.
// If the number of child nodes exceeds this total, we'll need to allocate more storage,
// so there's a balance between not wasting too much space versus the cost of reallocation
//
pub const MAX_EXPECTED_CHILD_MOVES: usize = 40;
