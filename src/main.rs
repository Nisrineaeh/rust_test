use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    data_type();
    flux_controle(10);
    addition(10, 5);
    tuple();
}

// fn mutuality(){
//     let x = 5; // immuable
//     let mut y = 5; // mutable
// }

fn data_type(){
    let _entier: i32 = 5;
    let _flottant: f64 = 3.14;
    let _bool: bool = true;
    let _chaine: String = String::from("Hello, Rust!");
    let _tuple: (i32, f64, bool) = (42, 3.14, false);
    let _tableau: [i32; 5] = [1, 2, 3, 4, 5];
}

fn tuple(){
      let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup; 
    println!("La valeur de y est : {}", y);

    let premier_element = tup.0;
    let deuxieme_element = tup.1;
    let troisieme_element = tup.2;

    println!("Le premier élément est : {}", premier_element);
    println!("Le deuxième élément est : {}", deuxieme_element);
    println!("Le troisième élément est : {}", troisieme_element);
}

fn flux_controle(x:i32){
    if x > 5 {
    println!("x est plus grand que 5");
}

for i in 0..x {
    println!("{}", i);
}
}

fn addition(a: i32, b: i32) {
    let resultat = a + b;
    println!("Le résultat de l'addition de {} et {} est {}", a, b, resultat); // INCROYABLE !
}

