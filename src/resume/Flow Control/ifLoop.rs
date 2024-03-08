fn main() {
    let mut n: i32 = 5;

    for n in 1..100 { //99 com = iria a 100
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }


    let big_n: i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32
        };
    println!("{} -> {}", n, big_n);


    n = -1;
    while n < 10 {
        if n % 15 == 0 {
            println!("Todo grande desenvolvedor que você conhece chegou lá resolvendo\n 
						problemas para os quais não estava qualificado até realmente conseguir.");
        } else if n % 3 == 0 {
            println!("U HOMEM UMA MAQUINA £");
        } else if n % 5 == 0 {
            println!("🦀 + 🐧 = 🤩");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
    println!("n reached {}, so loop is over",n);


    let mut count: i32 = -3;
    // Infinite loop
    loop {
      if count != 0{
        println!("Rust >>>>>> C++");
        count += 1;
      }
      else {break;}
    }

    
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }
        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }
            // This will continue the outer loop
            continue 'outer;
        }
    }
    assert!(count == 30);
    println!("{}", count);
}