use std::mem::size_of_val;
fn main(){

  let c3: char = '🦀'; //or 中 or L = 4 bytes 
  assert_eq!(size_of_val(&c3), 4);

  let unit: () = (); //null value
  assert!(size_of_val(&unit) == 0);

  let s = sum(1 , 2);
  print();
  panic!("aaaaaaaaaaaa") //interrompe a execução do programa.
}

fn sum(x: i32, y: i32) -> i32 { //-> especifica o tipo de retorno
    x + y
}

fn print() -> () { // -> () nenhum argumento 
   println!("UUUUU");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }               //tipo o swift
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// ! (nunca) é um tipo especial representa uma função que nunca retorna, indicando um comportamento imprevisível ou a interrupção abrupta
fn never_return_fn() -> ! {
    panic!() // or 'unimplemented!()' or 'todo()'
}