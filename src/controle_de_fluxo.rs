
pub fn controles() {
    let arr = vec![10, 20, 30, 40, 50];

    for elemento in arr.iter(){
        println!("Elemento: {}", elemento);
    }

    for i in 0..arr.len() {
        println!("Elemento: {}", arr[i]);
    }

    for i in (0..arr.len()).rev() {
        println!("Elemento: {}", arr[i]);
    }
}