use rand::Rng;
use std::cmp::Ordering;

pub fn game(){
    println!("++ Advinhe o número ++");
    let numero_aleatorio: u8 = rand::thread_rng().gen_range(0..101);
    loop {
        eprint!("Digite seu palpite: ");
        let mut palpite = String::new();
        std::io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a entrada.");
        let palpite: u8 = match palpite
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match palpite.cmp(&numero_aleatorio) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            },
        }
    }
}