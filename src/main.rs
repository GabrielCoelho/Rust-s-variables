fn main() {
    let x = five();

    println!("{x}");
}

// Por padrão, funções retornam a última linha. 
// Podemos também retornar algo via `return`
fn five() -> i32 {
    5
}
