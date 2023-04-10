pub mod vegetables;

use crate::garden::vegetables::Asparagus;

// inline mod

#[allow(dead_code)]
mod fruits {
    enum Kind {
        Apple,
        Orange,
    }
}

#[allow(dead_code)]
fn use_code_from_vegetables_mod() {
    let aspargus = Asparagus { size: 10 };

    println!("Yesterday I harvested an {:?}", aspargus);
}
