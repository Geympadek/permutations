use crate::permutations::permutation_from_idx;

mod permutations;

fn main() {
    //множество
    let mut foo = vec![0, 1, 2];

    //Выводит индекс перестановки, где множество это первый аргумент
    println!("{}", permutations::idx_from_permutation(&foo, &[2, 1, 0]));

    //Переставляет элементы в множестве по данному индексу
    permutation_from_idx(&mut foo, 5);
    println!("{:?}", foo);
}
