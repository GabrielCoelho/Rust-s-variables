fn main() {
    // cria uma expressão a partir de um novo escopo onde, a variável y terá seu valor atribuido
    // pelo resultado desta expresão.
    let y = {
        // Expresão que irá retornar 4, pois x = 3 e, 3+1 = 4.
        let x = 3;
        // note que esta última linha não tem o ponto e vírgula. Pois, se tivesse, seria um
        // "Statement" e não uma expressão. Esse valor será atribuido para y. Já a linha acima,
        // possue um ; para declarar que aquilo não será atribuído para o y. 
        x+1
    };

    println!("O valor de y é {y}");
}
