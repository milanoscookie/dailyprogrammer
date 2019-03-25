// https://www.reddit.com/r/dailyprogrammer/comments/b0nuoh/20190313_challenge_376_intermediate_the_revised/
    
static TEST:&[(u64, u64)] = &[
    (2016, 2017),
    (2019, 2020),
    (1900, 1901),
    (2000, 2001),
    (2800, 2801),
    (123456, 123456),
    (1234, 5678),
    (123456, 7891011),
    (123456789101112, 1314151617181920)
];


fn num_of_leaps(y:u64) -> u64 {
    y / 4 - y / 100 + ( y / 900 ) * 2 + match y % 900 {
        200...599 => 1,
        600...899 => 2,
        _ => 0
    }
}

fn leaps(x : u64, y : u64) -> u64 {
    num_of_leaps(y - 1) - num_of_leaps(x - 1)
} 
fn main() {
    for &(x, y) in TEST {
        println!("leaps({}, {}) => {}", x, y, leaps(x, y));
    }
}
