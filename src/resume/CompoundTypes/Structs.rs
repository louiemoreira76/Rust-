#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32, i32);

fn main(){
    let age: u8 = 19;
    let u1: User = User { // só com mut para mudar depois de criado!
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2: User = User{
        email: String::from("contact@im.dev"),
        ..u1 //u1
    };

    let v: Point = Point(0, 127, 255);

    dbg!(&u2); //exibirá uma representação formatada da estrutura, incluindo os valores de seus campos.
    println!("{:?}", u2);
} 