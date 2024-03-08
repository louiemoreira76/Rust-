fn main() {
    let _y: i32;
    let mut x: i32 = 1; //variavel mutavel
    x += 2;

  {
      let soma: i32 = x * 10;  
      println!("The value of x is {}", x); //Scope(escopo)
      println!("{soma}");       
  }

    assert_eq!(x, 3); // verificação de de valor
    println!("Success!");

    let z: bool = {
      let _t: bool = true && false; // false
      _t = true || false; // true
    };

   define_x();
  array();
}

fn define_x() {
    let x: &str = "Luis Rust dev";// Declaração da variável x como uma string

    println!("{}, world", x);
  
let y: i32 = 4;// Declaração de uma variável y imutável(pq não tem o mut) e inicialização com o valor 4

    // Shadowing (Sombreamento)
let y: &str = "I can also be bound to text!";//sombreia a variável anterior
  println!("{}", y);

  let (_a, _b) = (1, 2); // Declaração de duas variáveis
}

fn array() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}