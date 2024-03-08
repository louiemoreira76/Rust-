enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"), //representa um padrão curinga, que corresponde a qualquer valor que não tenha sido especificado anteriormente
    };

    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))//para verificar se o caractere ab corresponde a qualquer uma das seguintes faixas: letras maiúsculas de 'A' a 'Z', letras minúsculas de 'a' a 'z' ou dígitos de '0' a '9'
    }
}