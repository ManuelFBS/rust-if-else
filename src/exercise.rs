#[derive(PartialEq, Debug)]
struct Car {
        color: String, motor: Transmission, roof: bool, age: (Age, u32)
}

#[derive(PartialEq, Debug)]
enum Transmission {
        Manual, SemiAuto, Automatic
}

#[derive(PartialEq, Debug)]
enum Age {
        New, Used
}

////////////////////////////////////////////////////////////////////////

fn car_quality (miles: u32) -> (Age, u32) {
        if miles > 0 {
        // El automóvil tiene millas acumuladas, de manera 
        // que es usado...
                return (Age::Used, miles);
        } else {
                return (Age::New, 0);
        }
}

///////////////////////////////////////////////////////////////////////

// Contruir un Automóvil nuevo, usanddo los valores de cuatro
// argumntos de entrada...
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Se llama a la función car_quality(miles) para obtener 
// el 'car age'
// Devuelve una instancia de la "Car" struct con la sintáxis `->`...
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Se verifica si el automóvil es nuevo o usado y se imprime
    // los detalles...
        let car_age = car_quality(miles);

        if car_age.0 == Age::Used {
        // Si el automóvil es usado, se verifica el tipo de techo...
                if roof {
                        println!("Preparar un carro usado: {:?}, Transmission: {:?}, Hard top, {} miles\n", color, motor, miles);
                } else {
                        println!("Preparar un carro usado: {:?}, Transmission: {:?}, Convertible, {} miles\n", color, motor, miles);
                }
        } else {
        // El automóvil es nuevo, verificar el tipo dde techo...
                if roof {
                        println!("Preparar un carro nuevo: {:?}, Transmission: {:?}, Hard top, 0 miles\n", color, motor);
                } else {
                        println!("Preparar un carro nuevo: {:?}, Transmission: {:?}, Convertible, 0 miles\n", color, motor);
                }
        }

    // Se crea una nueva instancia de Car la cual es requerida...
    // - Se unen los 3 primeros campos con los valores de los
    // argumentos de entrada...
    // - Se une "age" a la tupla devuelta desde car_quality(miles)...
        Car {
                color,
                motor,
                roof,
                age: car_age
        }
}

fn main() {
        car_factory(String::from("Orange"), Transmission::Manual, true, 0);
        car_factory(String::from("Red"), Transmission::SemiAuto, false, 15800);
        car_factory(String::from("Silver"), Transmission::Automatic, true, 25000);
}