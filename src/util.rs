/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */

pub(crate) trait SemanticEq {
    fn semantic_eq(&self, other: &Self) -> bool;
    fn semantic_neq(&self, other: &Self) -> bool {
        !self.semantic_eq(other)
    }
}

pub(crate) struct Permutations<T> {
    elements: Vec<T>,
    state: usize,
}
// // 0 [0, 1, 2] -> [0, 1, 2]
// // 1 [0, 1, 2] -> [0, 2, 1]
// // 2 [0, 1, 2] -> [1, 0, 2]
// // 3 [0, 1, 2] -> [1, 2, 0]
// // 4 [0, 1, 2] -> [2, 0, 1]
// // 5 [0, 1, 2] -> [2, 1, 0]

// // 0 [0, 1, 2] % 3 = 0
// // 3 [0, 2, 1] % 3 = 0
// // 1 [1, 0, 2] % 3 = 1
// // 4 [1, 2, 0] % 3 = 1
// // 2 [2, 0, 1] % 3 = 2
// // 5 [2, 1, 0] % 3 = 2

// // 0 [0, 1, 2] % 2 = 0
// // 2 [2, 0, 1] % 2 = 0
// // 4 [1, 2, 0] % 2 = 0
// // 1 [1, 0, 2] % 2 = 1
// // 3 [0, 2, 1] % 2 = 1
// // 5 [2, 1, 0] % 2 = 1
// fn static_permutation<I: std::fmt::Debug + Clone>(elems: &[I], n: usize) -> [I; 3] {
//     let first_elem = n % 3;
//     match n {
//         0 => {[
//             elems[0].clone(),
//             elems[1].clone(),
//             elems[2].clone(),
//         ]},
//         1 => {[
//             elems[1].clone(),
//             elems[0].clone(),
//             elems[2].clone(),
//         ]},
//         2 => {[
//             elems[2].clone(),
//             elems[0].clone(),
//             elems[1].clone(),
//         ]},
//         3 => {[
//             elems[0].clone(),
//             elems[2].clone(),
//             elems[1].clone(),
//         ]},
//         4 => {[
//             elems[1].clone(),
//             elems[2].clone(),
//             elems[0].clone(),
//         ]},
//         5 => {[
//             elems[2].clone(),
//             elems[1].clone(),
//             elems[0].clone(),
//         ]},
//         _ => panic!(),
//     }
// }
// fn heap_permutation<I: std::fmt::Debug>(elems: &mut Vec<I>, size: usize) {
//     assert!(size <= elems.len());
//     if size == 1 {
//         println!("{:?}", elems);
//     } else {
//         for i in 0..size {
//             heap_permutation(elems, size - 1);
//             if size % 2 == 1 {
//                 println!("Swapping {} and {}", 0, size - 1);
//                 elems.swap(0, size - 1);
//             } else {
//                 println!("Swapping {} and {}", i, size - 1);
//                 elems.swap(i, size - 1);
//             }
//         }
//     }
// }
// #[test]
// fn heap_perm() {
//     let mut vec = vec![0, 1, 2];
//     let size = vec.len();
//     heap_permutation(&mut vec, size);
//     panic!();
// }
