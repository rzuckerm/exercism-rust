pub fn verse(n: u32) -> String {
    let (first_count, second_count) = match n {
        0 => ("No more bottles".to_owned(), "no more bottles".to_owned()),
        1 => ("1 bottle".to_owned(), "1 bottle".to_owned()),
        _ => (format!("{n} bottles"), format!("{n} bottles")),
    };
    let second_phrase = match n {
        0 => "Go to the store and buy some more, 99 bottles".to_owned(),
        1 => "Take it down and pass it around, no more bottles".to_owned(),
        2 => "Take one down and pass it around, 1 bottle".to_owned(),
        _ => format!("Take one down and pass it around, {} bottles", n - 1),
    };
    format!(
        "\
{first_count} of beer on the wall, {second_count} of beer.
{second_phrase} of beer on the wall.\n"
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .into_iter()
        .collect::<Vec<String>>()
        .join("\n")
}
