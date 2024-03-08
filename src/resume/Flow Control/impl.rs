#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    edge: u32,
    base: u32
}
#[derive(Debug)]
enum Name{
    Fname,
    Sname,
    Lname,
}

struct TrafficLight {
    color: String,
}
//diferença entre atribuição de valor no Rectangle(Struct) e Name(Enum)
impl Rectangle {

    fn n(&self) -> u32{
       
        if self.edge == 4 {
            return 4;
        } else if self.base == 1 {
            return 1;
        }
        0 
    }

    fn area(&self) -> u32 { 
        self.width * self.height
    }
}

impl Name {
    fn Nmas(&self) -> &str{
        match self{
            Self::Fname => "Luis",
            Self::Sname => "Felipi",
            Self::Lname => "Moreira",
        }
    }
}

impl TrafficLight {
    pub fn new() -> Self{
        Self{  //função
            color: String::from("red"),
        } 
     }

     pub fn show_state(&self)  { // metodo
        println!("the current state is {}", self.color);
    }

    pub fn change_state(other: &mut TrafficLight)  { //função
        other.color = "green".to_string()
    }

    pub fn get_state(&self) -> &str { //metodo
        &self.color 
    }
}

fn main() {
    let rect1: Rectangle = Rectangle { width: 30, height: 50, edge: 4, base: 1 }; 

    let mut light: TrafficLight = TrafficLight::new(); 
    light.show_state();
    TrafficLight::change_state(&mut light); 

    assert_eq!(light.get_state(), "green");
    assert_eq!(rect1.area(), 1500);

    let name: Name = Name::Fname;

    println!("{:?}, {:?}", name, name.Nmas());
}
