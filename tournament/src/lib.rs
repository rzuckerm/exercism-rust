use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut stats_map: HashMap<String, Vec<i32>> = HashMap::new();
    for line in match_results.lines() {
        let mut iter = line.split(';');
        if let (Some(name1), Some(name2), Some(outcome)) = (iter.next(), iter.next(), iter.next()) {
            let (index1, index2): (usize, usize) = match outcome {
                "win" => (1, 3),  // team 1 win, team 2 loss
                "loss" => (3, 1), // team 2 win, team 1 loss
                _ => (2, 2),      // team 1 draw, team 2 draw
            };
            update_stat(&mut stats_map, name1, index1);
            update_stat(&mut stats_map, name2, index2);
        }
    }

    let mut stats: Vec<(String, Vec<i32>)> = stats_map.into_iter().collect();
    stats.sort_by_key(|(name, stat)| (-stat[4], name.to_owned()));
    stats.iter().fold(
        "Team                           | MP |  W |  D |  L |  P".to_owned(),
        |acc, (name, stat)| {
            acc + &format!(
                "\n{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                name, stat[0], stat[1], stat[2], stat[3], stat[4],
            )
        },
    )
}

fn update_stat(stats_map: &mut HashMap<String, Vec<i32>>, name: &str, index: usize) {
    let entry = stats_map
        .entry(name.to_owned().to_string())
        .or_insert(vec![0, 0, 0, 0, 0]);
    entry[index] += 1;
    entry[0] += 1; // Matches played
    entry[4] = 3 * entry[1] + entry[2]; // Score = 3 * wins + draws
}
