pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    match input.len() {
        0 => Some(vec![]),
        1 => (input[0].0 == input[0].1).then_some(vec![input[0]]),
        _ => {
            let chainables = get_chainables(input)?;
            let mut visited = vec![false; input.len()];
            let mut output = Vec::<(u8, u8)>::new();
            find_first_chain(0, input, &chainables, &mut visited, &mut output).then_some(output)
        }
    }
}

fn get_chainables(input: &[(u8, u8)]) -> Option<Vec<Vec<usize>>> {
    let mut chainables = vec![vec![]; 2 * input.len()];
    for (i, &(first1, second1)) in input.iter().enumerate() {
        for (second, index) in [(second1, 2 * i), (first1, 2 * i + 1)] {
            for (j, &(first2, second2)) in input.iter().enumerate() {
                if i != j && (second == first2 || second == second2) {
                    chainables[index].push(2 * j + (second == second2) as usize);
                }
            }

            chainables[index].first()?;
        }
    }

    Some(chainables)
}

fn find_first_chain(
    key: usize,
    input: &[(u8, u8)],
    chainables: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    output: &mut Vec<(u8, u8)>,
) -> bool {
    visited[key / 2] = true;
    match key % 2 {
        0 => output.push(input[key / 2]),
        _ => output.push((input[key / 2].1, input[key / 2].0)),
    }

    if output.len() == input.len() {
        return output[0].0 == output.last().unwrap().1;
    }

    for next_key in chainables[key].clone() {
        if !visited[next_key / 2] && find_first_chain(next_key, input, chainables, visited, output)
        {
            return true;
        }
    }

    visited[key / 2] = false;
    output.pop();
    false
}
