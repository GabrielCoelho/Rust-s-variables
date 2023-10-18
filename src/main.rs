fn main() {
    let mut counter = 0;

    // Retornando valores via Loop 
    let result = loop {
        counter += 1;

        if counter == 10 {
            // Se chegar na condição, quebra o loop retornando o contador x 2
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
