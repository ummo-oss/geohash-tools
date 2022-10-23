use phf::{phf_map, Map};

/// Rune of possible GeoHash characters. Values (char) are mapped to their relative index (usize) in the rune.
pub static RUNE_CHAR_INDEX: Map<char, usize> = phf_map! {
    '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5, '6' => 6, '7' => 7, '8' => 8, '9' => 9,
    'b' => 10, 'c' => 11, 'd' => 12, 'e' => 13, 'f' => 14, 'g' => 15, 'h' => 16, 'j' => 17, 'k' => 18, 'm' => 19,
    'n' => 20, 'p' => 21, 'q' => 22, 'r' => 23, 's' => 24, 't' => 25, 'u' => 26, 'v' => 27, 'w' => 28, 'x' => 29,
    'y' => 30, 'z' => 31,
};

/// Rune of possible GeoHash characters. Relative indexes (static str) in the rune are mapped to their values (char).
pub static RUNE_INDEX_CHAR: Map<&'static str, char> = phf_map! {
    "0" => '0', "1" => '1', "2" => '2', "3" => '3', "4" => '4', "5" => '5', "6" => '6', "7" => '7', "8" => '8', "9" => '9',
    "10" => 'b', "11" => 'c', "12" => 'd', "13" => 'e', "14" => 'f', "15" => 'g', "16" => 'h', "17" => 'j', "18" => 'k', "19" => 'm',
    "20" => 'n', "21" => 'p', "22" => 'q', "23" => 'r', "24" => 's', "25" => 't', "26" => 'u', "27" => 'v', "28" => 'w', "29" => 'x',
    "30" => 'y', "31" => 'z',
};

/// Length of rune of possible GeoHash characters.
pub const RUNE_LEN: usize = 32;

/// Length of rune of possible GeoHash characters -1 to represent the largest index in the rune.
pub const RUNE_INDEX_LEN: usize = 31;

pub static GEOHASH_ID_RANGE: [i32; 13] = [
    0,
    geohash_id_range(1),
    geohash_id_range(2),
    geohash_id_range(3),
    geohash_id_range(4),
    geohash_id_range(5),
    geohash_id_range(6),
    geohash_id_range(7),
    geohash_id_range(8),
    geohash_id_range(9),
    geohash_id_range(10),
    geohash_id_range(11),
    geohash_id_range(12),
];

const fn geohash_id_range(precision: u32) -> i32 {
    RUNE_LEN.pow(precision) as i32
}