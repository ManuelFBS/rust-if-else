// INDEXACION DE UN VECTOR...
fn main() {
  // Se declara un vector con 3 valores...
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];

    println!("Vector: {:?}, three = {}", index_vec, three);

    index_vec[1] = index_vec[1] + 7;

    println!("Vector: {:?}", index_vec);
}