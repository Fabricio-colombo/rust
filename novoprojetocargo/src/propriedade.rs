fn main() {
    let mut s = String::from("hello"); // s se torna o proprietário da string.

    {
        let r1 = &s; // Um empréstimo imutável de s.
        let r2 = &s; // Outro empréstimo imutável de s. Isso é permitido.
        println!("{} and {}", r1, r2);
        // r1 e r2 podem ser usados aqui.
    } // r1 e r2 saem do escopo aqui, mas s não é afetado.

    {
        let r3 = &mut s; // Um empréstimo mutável de s.
        r3.push_str(", world");
        println!("{}", r3);
        // Somente um empréstimo mutável é permitido em um escopo, para evitar condições de corrida.
    } // r3 sai do escopo aqui.

    // Se tentarmos usar r1, r2, ou r3 aqui, teremos um erro de compilação.
    // s é descartado aqui, pois sai do escopo.
}
