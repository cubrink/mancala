use crate::mancala::WalkData;

pub fn get_precomputed_walk(start: usize, steps: usize) -> Option<WalkData> {
    let idx: usize = 48 * (start - 1) + (steps - 1);
    PRECOMPUTED_WALK[idx]
}

pub static PRECOMPUTED_WALK: [Option<WalkData>; 624] = [
    Some(WalkData {
        visited: [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 3, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 3, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 3, 3, 3, 3, 3, 3],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 3, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 4, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 3, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 2, 2, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3],
        final_idx: 7,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [0, 4, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [0, 4, 4, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4],
        final_idx: 2,
    }),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [2, 1, 1, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [2, 2, 1, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 1, 1, 1, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 1, 1, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 1, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 1, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [3, 2, 2, 2, 2, 2, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [3, 3, 2, 2, 2, 2, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 2, 2, 2, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 2, 2, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 2, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 2, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 2, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 4, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 4, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 4, 4, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 4, 4, 4, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [4, 3, 3, 3, 3, 3, 3, 0, 3, 4, 4, 4, 4, 4],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [4, 4, 3, 3, 3, 3, 3, 0, 3, 4, 4, 4, 4, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 3, 3, 3, 3, 0, 3, 4, 4, 4, 4, 4],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 3, 3, 3, 0, 3, 4, 4, 4, 4, 4],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [2, 2, 1, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 1, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 1, 1, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 1, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 1, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 1, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [3, 2, 2, 2, 2, 2, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [3, 3, 2, 2, 2, 2, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 2, 2, 2, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 2, 2, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 2, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 2, 2, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 2, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 4, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 4, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 4, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 4, 4, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [4, 3, 3, 3, 3, 3, 3, 0, 3, 3, 4, 4, 4, 4],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [4, 4, 3, 3, 3, 3, 3, 0, 3, 3, 4, 4, 4, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 3, 3, 3, 3, 0, 3, 3, 4, 4, 4, 4],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 3, 3, 3, 0, 3, 3, 4, 4, 4, 4],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 3, 3, 0, 3, 3, 4, 4, 4, 4],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [2, 2, 1, 1, 1, 1, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 1, 1, 1, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 1, 1, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 1, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [3, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [3, 3, 2, 2, 2, 2, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 2, 2, 2, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 2, 2, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 2, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 2, 2, 2, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 2, 2, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 2, 3, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 4, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 4, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [4, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [4, 4, 3, 3, 3, 3, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 3, 3, 3, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 3, 3, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 3, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 3, 0, 3, 3, 3, 4, 4, 4],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [2, 2, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 1, 1, 1, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 1, 1, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 1, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 1, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 1, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [3, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [3, 3, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 2, 2, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 2, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 2, 2, 2, 2, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 2, 2, 2, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 2, 2, 3, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 2, 3, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 4, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [4, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [4, 4, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 3, 3, 3, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 3, 3, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 3, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 3, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 4, 0, 3, 3, 3, 3, 4, 4],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [2, 2, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 1, 1, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 1, 1, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 1, 1, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 1, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [3, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [3, 3, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 2, 2, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 2, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 2, 2, 2, 2, 2, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 2, 2, 2, 2, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 2, 2, 2, 3],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 2, 2, 3],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 2, 3],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [4, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [4, 4, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 3, 3, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 3, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 3, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 4, 0, 3, 3, 3, 3, 3, 4],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 4, 0, 4, 3, 3, 3, 3, 4],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [2, 2, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 1, 1, 1],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 1, 1, 1],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 1, 1],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 1],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [3, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [3, 3, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 2, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 2, 2, 2, 2, 2, 2],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 2, 2, 2, 2, 2],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 2, 2, 2, 2],
        final_idx: 9,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 2, 2, 2],
        final_idx: 10,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 2, 2],
        final_idx: 11,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 2],
        final_idx: 12,
    }),
    Some(WalkData {
        visited: [3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 13,
    }),
    Some(WalkData {
        visited: [4, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 0,
    }),
    Some(WalkData {
        visited: [4, 4, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 1,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 2,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 3,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 3, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 4,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 3, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 5,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 4, 0, 3, 3, 3, 3, 3, 3],
        final_idx: 6,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 4, 0, 4, 3, 3, 3, 3, 3],
        final_idx: 8,
    }),
    Some(WalkData {
        visited: [4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 3, 3, 3, 3],
        final_idx: 9,
    }),
];
