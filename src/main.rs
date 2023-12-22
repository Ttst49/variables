const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;


fn var() {
    let mut x = 5;
    println!("La valeur de x est : {}", x);
    x = 6;
    println!("La valeur de x est : {}", x);
}

use std::io;

fn tableaux() {
    let a = [1, 2, 3, 4, 5];

    println!("Veuillez entrer un indice de tableau.");

    let mut indice = String::new();

    io::stdin()
        .read_line(&mut indice)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let indice: usize = indice
        .trim()
        .parse()
        .expect("L'indice entré n'est pas un nombre");

    let element = a[indice];

    println!(
        "La valeur de l'élément d'indice {} est : {}",
        indice, element
    );
}


fn main(){
    tableaux();
}