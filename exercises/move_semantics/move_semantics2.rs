// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // Solution 1: Clone the `vec0` and pass the cloned one to `fill_vec`.
    // let vec0 = Vec::new();
    // let vec0_ = vec0.clone();
    // let mut vec1 = fill_vec(vec0_);

    // Solution 2: Pass the immutable reference of `vec0` to `fill_vec`.
    // let vec0 = Vec::new();
    // let mut vec1 = fill_vec(&vec0);

    // Solution 3: Make `vec0` mutable and modify it inside the `fill_vec`.
    let mut vec0 = Vec::new();
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // For Solution 1 and Solution 2
    // vec1.push(88);
    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // For Solution 3
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

// Solution1: Take ownership of `vec` and make it mutable,
// then modify the mutable one, finally return it.
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// Solution2: Borrow `vec` and copy it, then modify the copied one, and return it.
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// Solution3: Borrow mutably `vec`, then modify the mutable reference.
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
