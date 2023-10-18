fn main() {
    // Define uma variável semi-constante valendo 5
    let x = 5
    // Sombreamento da variavel x, definindo x como o sombreado + 1
    let x = x + 1;
    // cria um escopo (particionamento do código)
    {
        // dentro deste escopo, sombrea o x mais recente, para que ele passe a valer ele mesmo x 2.
        let x = x * 2;
        // printa na tela o resultado = 12
        println!("O Valor de x dentro deste escopo é: {x}");
    }
    // saindo do escopo, nós voltamos ao valor sombreado mais recente. Como o primeiro valor e o
    // mais recente estão dentro da mesma função main, será sempre o valor de 5+1 = 6.
    println!("O valor de x é: {x}");
}
