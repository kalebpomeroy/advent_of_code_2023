// A collection of useful, generic vector things that rust doesn't have 

pub fn prepend_to_vector<T>(item: T, mut original: Vec<T>) -> Vec<T> {
    original.insert(0, item);
    original
}
