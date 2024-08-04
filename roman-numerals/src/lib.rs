pub struct Roman;

#[rustfmt::skip]
static ROMAN_TABLE: [(usize, [&str; 10]); 4] = [
    (1000, ["", "M", "MM", "MMM", "", "", "", "", "", ""]),
    (100,  ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]),
    (10,   ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"]),
    (1,    ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"]),
];

impl Roman {
    pub fn from(num: u32) -> String {
        ROMAN_TABLE
            .iter()
            .map(|(n, v)| v[(num as usize / n) % 10])
            .collect()
    }
}
