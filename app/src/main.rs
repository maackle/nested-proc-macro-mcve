use macros::{noop, quoter};

fn main() {
    alpha();
    beta();
    gamma();
    delta();
    epsilon();
}

#[noop]
fn alpha() {
    let _ = 1 + 1.0;
}

#[quoter]
fn beta() {
    let _ = 1 + 1.0;
}

// This is the only combination that causes line info to be erased
#[noop]
#[quoter]
fn gamma() {
    let _ = 1 + 1.0;
}

#[quoter]
#[noop]
fn delta() {
    let _ = 1 + 1.0;
}

#[quoter]
#[quoter]
fn epsilon() {
    let _ = 1 + 1.0;
}
