use std::collections::{HashMap, HashSet};

// (row, column)
type Point = (usize, usize);

// Set of points connected to a specific point:
// - For a horizon line, this is a set of column numbers
// - For a vertical line, this is a set of row numbers
type PointsInLine = HashSet<usize>;

#[derive(Default, Debug)]
struct Connections {
    h_conns: PointsInLine, // Horizontal connections
    v_conns: PointsInLine, // Vertical connections
}

pub fn count(lines: &[&str]) -> u32 {
    let lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    // Looking right, find all horizontal connections among points ('+').
    // A connection between two points starts and ends with '+' and is
    // connected by zero of more '+' or '-'. The connect ends on any other
    // character
    let mut conns = HashMap::<Point, Connections>::new();
    for (r, line) in lines.iter().enumerate() {
        let mut curr_col: Option<usize> = None;
        for (c, &ch) in line.iter().enumerate() {
            if ch == '+' {
                conns.insert((r, c), Connections::default());
                match curr_col {
                    None => curr_col = Some(c),
                    Some(curr_col) => {
                        conns.get_mut(&(r, curr_col)).unwrap().h_conns.insert(c);
                        for c2 in conns[&(r, curr_col)].h_conns.clone() {
                            if c2 != c {
                                conns.get_mut(&(r, c2)).unwrap().h_conns.insert(c);
                            }
                        }
                    }
                }
            } else if ch != '-' {
                curr_col = None
            }
        }
    }

    // Looking down, find all vertical connections among points ('+').
    // A connection between two points starts and ends with '+' and is
    // connected by zero of more '+' or '|'. The connect ends on any other
    // character
    let num_cols = lines.iter().fold(0, |acc, line| acc.max(line.len()));
    for c in 0..num_cols {
        let mut curr_row: Option<usize> = None;
        for (r, line) in lines.iter().enumerate() {
            let &ch = line.get(c).unwrap_or(&' ');
            if ch == '+' {
                match curr_row {
                    None => curr_row = Some(r),
                    Some(curr_row) => {
                        conns.get_mut(&(curr_row, c)).unwrap().v_conns.insert(r);
                        for r2 in conns[&(curr_row, c)].v_conns.clone() {
                            if r2 != r {
                                conns.get_mut(&(r2, c)).unwrap().v_conns.insert(r);
                            }
                        }
                    }
                }
            } else if ch != '|' {
                curr_row = None;
            }
        }
    }

    // Count rectangles
    //
    //    a            b
    // (r1, c1) --- (r1, c2)
    //    |            |
    // (r2, c1) --- (r2, c2)
    //    c            d
    //
    // Rectangle if all the following are true:
    // - Point d exists
    // - Point b and d are connected
    // - Point c and d are connected
    println!("{:?}", conns);
    let mut count = 0;
    for ((r1, c1), pt_conns) in &conns {
        for r2 in pt_conns.v_conns.clone() {
            for c2 in pt_conns.h_conns.clone() {
                if conns.contains_key(&(r2, c2))
                    && conns[&(*r1, c2)].v_conns.contains(&r2)
                    && conns[&(r2, *c1)].h_conns.contains(&c2)
                {
                    count += 1;
                }
            }
        }
    }

    count
}
