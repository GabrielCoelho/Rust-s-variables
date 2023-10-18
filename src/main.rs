fn main() {
    // Cria uma variavel do tipo String atribuindo vários espaços à ela. 
    let spaces = "   ";
    // Sombrea a primeira  a partir de uma nova variável do tipo Inteiro
    // isso não funcionaria se na linha 3 fosse criado uma variavel mut, 
        // pois estaríamos dentro dela tentando mudar o texto com um valor inteiro. 
    let spaces = spaces.len();
    println!("o número de espaços contido é de: {spaces}");
}
