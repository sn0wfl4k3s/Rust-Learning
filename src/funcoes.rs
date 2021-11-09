use std::cmp::Ordering;

fn fibonacci (pos: u16) -> u64 {
    match pos.cmp(&3) {
        Ordering::Less => 1,
        _ => fibonacci(pos - 1) + fibonacci(pos -2)
    }
}

pub fn fibonacci_exec() {
    loop {
        let mut n = String::new();
        println!("Digite o número: ");
        std::io::stdin().read_line(&mut n).expect("Falha ao tentar ler.");
        if n.trim().to_lowercase() == "quit" {
            break;
        }
        let n= match n.trim().parse::<u16>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Resultado: {}", fibonacci(n));
    }
}