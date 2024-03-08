use std::ops::{Range, RangeInclusive};

fn main() {
  assert_eq!(i8::MAX, 127);
  assert_eq!(u8::MAX, 255);

  let v: u16 = 38_u32 as u16; // para garantir que o tipo fica em u16
  println!("{}", v);

  assert_eq!("u16".to_string(), type_of(&v));// conversão 

  println!("{}", v); // só que string

  somaIeU();
  float();
  Range();
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn somaIeU(){
  let v1: u16 = 251_u16 + 8; // de baixo é mesma coisa
   let v2: i16 = i16::checked_add(251, 8).unwrap();//Se houver um estouro, o   programa irá falhar.
  let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
   println!("{}, {}, {}", v1, v2, v);
}

fn float(){
  let x: f64 = 1_000.000_1;
  let y: f32 = 0.12;
  let z: f64 = 0.01_f64;
}

fn Range(){
  assert_eq!((1..5), Range { start: 1, end: 5 }); // 5 não incluso(exclusivo)
  assert_eq!((1..=5), RangeInclusive::new(1, 5)); // 5 incluso

  println!("Success!");
}