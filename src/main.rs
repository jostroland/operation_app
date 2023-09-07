extern crate core;

use std::io;

fn main() {
    println!("Entrer votre premier nombre entier");
    let a: i32 = read_var(&mut String::new())
                    .expect("Erreur de donnees saisies");

    println!("Entrer votre second nombre entier");
    let b: i32 = read_var(&mut String::new())
                    .expect("Erreur de donnees saisies");

    let res = division(a, b);
    match res {
         Ok(res) =>  println!("Le resultat est : {}",res),
         Err(error) => println!("{}", error),
    }
}

fn read_var(value: &mut String) -> Result<i32, &str> {
    match io::stdin().read_line(value) {
        Ok(_n) => match value.trim().parse::<i32>() {
            Ok(r) => Ok(r),
            Err(error) => panic!("{}", error),
        },
        Err(error) => panic!("{}", error),
    }
}

fn division(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        Err(String::from("Imppossible de diviser par zéro"))
    }
     Ok(a as f32 / b as f32)
}
