fn main(){
    let x: String = String::from("Hello World");
    let y: String = x.clone(); //cria uma cópia profunda q possue a propriedade completa dos dados
    let z: &String = &x; //criação de uma referência para x em vez de possuir a propriedade completa. 

    let n: (i32, i32, (), &str) = (1, 2, (), "hello"); // criação de tupla

    let mut p: Box<i32> = Box::new(1); // usa Box cria um ponteiro no heap
    *p = 4;  //usando o operador * atribui o valor 4
    assert_eq!(*p, 4);  // diz vá a esse endereço x39riejiw e devolva oq esta amarzenado 

    let ref r = x; //ref, que é uma referência de empréstimo para n

    println!("the memory address of p is {:p}, {}", p, r); //imprime o endereço de memória {:p}

}