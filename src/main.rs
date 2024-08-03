use std::fs::{self};


fn main() {
    let file = fs::read_to_string("./project/lines.txt").unwrap();

    file.lines().for_each(|line| println!("{}", line));
}

// let data = vec![1, 2, 3];
// let mut foo = data.iter().map(|x| x + 1);

// let combined_string: String = vec!["this", "is", "one", "string"].into_iter().collect();

// let my_hash_set: HashSet<isize> = vec![1, 1, 2, 3, 4].into_iter().collect();
// println!("here is my set: {:?}", my_hash_set);

// println!("combined string: { }", combined_string);

// let mut new_vector = vec![];

// while let Some(x) = foo.next() {
//     new_vector.push(x);
// }

// let my_hash_map: HashMap<usize, &str> = vec!["here", "is", "a", "hash", "map"].into_iter().enumerate().collect();

// println!("Here is the hash map: {:?}", my_hash_map);

// println!("here is x: {:?}", new_vector);
