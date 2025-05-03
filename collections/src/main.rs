mod hash_map;
mod string;
mod vector;

use crate::vector::reading_elements;

fn main() {
    let _v: Vec<i32> = Vec::new();

    let _v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    reading_elements();
}
