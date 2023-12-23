// Creación de un vector vacío, en caso de querer hacerlo
// más grande o más chico...
fn main() {
        // Se declara un vector vacío...
            let mut fruit = Vec::new();
    
        // Se agregan valores al final del vector, el type cambia
        // de generic `T` a String...
            fruit.push("Orange");
            fruit.push("Banana");
            fruit.push("Pineapple");
            fruit.push("apple");
    
            println!("Fruits: {:?}", fruit);
            println!("");
        // El método 'pop' elimina el último elemento del vector...
            println!("Pop off: {:?}", fruit.pop());
            println!("New Fruits: {:?}", fruit);
}