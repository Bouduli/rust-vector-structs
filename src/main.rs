use std::string;

#[derive(Debug)]
struct Car{
    brand : String, 
    model: String, 
    year: usize
    
        // color: String
}
fn clear(){
    //Some black magic shii to clear the console
    print!("\x1B[2J\x1B[1;1H");
}    

fn main() {
    
   


    let mut car_vector = vec![Car{ brand: String::from("BMW"), model: String::from("530"), year: 2003/* , color: String::from("Black") */}, 
    Car{brand : String::from("Audi"), model: String::from("A6 Avant"), year:2022},
    Car{brand: String::from("Volkswagen"), model: String::from("Shirroco R 2.0"), year: 2016}];

    //println!("{:#?}", car_vector);

    //CreateCar(&mut car_vector);
    //CreateCar(&mut car_vector);
    
    //DeleteCar(&mut car_vector);


    clear();
    println!("Cars in the car vector: ");
    ReadCar(&car_vector);
    UpdateCar(&mut car_vector);
     


}
//gets input from the console while trimming excess characters (pesky '\r')
fn getInput(msg : &str)->String{
    use std::io::{self, Write};
    let mut input:String = String::new();

    print!("{msg}: "); 
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Unable to read input"); 

    //Implicitly returns the string from stdin trimmed. 
    input.trim().to_string()



}

fn CreateCar(car_vector: &mut Vec<Car>){
    clear();

    println!("\n What brand is your car from? ");
    
    let brand = getInput("Brand");

    println!("\nWhat model is your car?");
    
    let model = getInput("Model");


    println!("\nWhat year was it made? ");

    let year = getInput("Year").parse::<usize>().unwrap();
    let new_car = Car{
        brand, 
        model, year
    };

    //Push takes ownership of new_car. 
    car_vector.push(new_car);


    clear();
    println!("Cars in the car vector\n");
    ReadCar(car_vector);    

}
fn ReadCar(car_vector: &Vec<Car>){
    
    
    for (i, car) in car_vector.iter().enumerate(){
        println!("{i}: {} {} {{...}}",car.brand, car.model );
    }
}
fn UpdateCar(car_vector: &mut Vec<Car>){

    clear();
    println!("What car would you like to update? \n");
    ReadCar(car_vector);

    
    let index = getInput("Index").parse::<u32>().unwrap();

    let car : &mut Car = &mut car_vector[index as usize];
    println!("\n\nWhat attributes do you wish to edit?\n\n");
    println!("b: Brand\nm: Model\ny:year");

   let attr = getInput("Attribute").chars().nth(0).unwrap();

   
    




}
fn DeleteCar(car_vector: &mut Vec<Car>){
    clear();
    println!("What Car would you like to remove?\n===============\n");
    ReadCar(car_vector);

    let inp = getInput("Index for deletion");
    let index = inp.parse::<u32>().unwrap();

    car_vector.remove(index as usize);


}