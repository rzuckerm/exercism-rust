#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let (capacities, other_bucket) = match *start_bucket {
        Bucket::One => ((capacity_1, capacity_2), Bucket::Two),
        Bucket::Two => ((capacity_2, capacity_1), Bucket::One),
    };
    let mut buckets = (capacities.0, 0u8);

    let mut moves = 1u8;
    while buckets.0 != goal && buckets.1 != goal && moves < u8::MAX {
        buckets = match buckets {
            (_, _) if capacities.1 == goal => (buckets.0, goal), // Goal met with other bucket
            (_, b) if b == capacities.1 => (buckets.0, 0),       // Empty other if full
            (0, _) => (capacities.0, buckets.1),                 // Fill start if empty
            (_, _) => {
                // Pour maximum amount from one bucket into the other
                let max_pour = buckets.0.min(capacities.1 - buckets.1);
                (buckets.0 - max_pour, buckets.1 + max_pour)
            }
        };
        moves += 1;
    }

    let (goal_bucket, other_bucket) = match (buckets.0 == goal, buckets.1 == goal) {
        (true, _) => (*start_bucket, buckets.1),
        (_, true) => (other_bucket, buckets.0),
        (_, _) => return None,
    };

    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}
