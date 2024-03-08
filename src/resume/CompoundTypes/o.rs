fn main(){
    let mut s: Box<str> = "hello, world".into(); //Box ponteiro para um valor na heap
    s.push_str('!');
    s.replace("Luis Rust");
    let unicode_codepoint = "\u{211D}";
    
    let list: [i32; 100] = [1; 100]; //todos com 1
    let arr: [_; 3] = ['a', 'b', 'c']; // tipo não especificado 
  
    assert!(std::mem::size_of_val(&arr) == 12); //tamanho do array arr é 12 bytes. char 3*4
    let arr0 = arr.get(0).unwrap();unwrap() //é usado para desembrulhar o valor

    //conceito de fatia(Slice)
    let arr: [char; 3] = ['中', '国', '人'];
    let slice: &[char] = &arr[..2];

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); // não pode >= 13
}