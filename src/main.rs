fn main() {
    let mut count = 0;
    // Atribui uma label com o nome deste loop externo
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        // cria um novo loop (interno) sem nome
        loop {

            println!("remaining = {remaining}");
            if remaining == 9 {
                // quebra o loop interno
                break;
            }
            if count == 2 {
                // quebra o loop externo, com o label.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
