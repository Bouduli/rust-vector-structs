#[derive(Debug, Clone)]
struct Car{
    model: String, 
    year: u32,
        // color: String
}
fn clear(){
    //Some black magic shii to clear the console
    print!("\x1B[2J\x1B[1;1H");
}    

fn main() {
    
   


    let mut car_vector = vec![Car{model: String::from("BMW"), year: 2003/* , color: String::from("Black") */}];

    println!("{:#?}", car_vector);

    CreateCar(&mut car_vector);
    
    

    

     


}
//gets input from the console while trimming excess characters (pesky '\r')
fn getInput()->String{
    let mut input:String = String::new();
    std::io::stdin().read_line(&mut input).expect("Unable to read input"); 

    //Implicitly returns the string from stdin trimmed. 
    input.trim().to_string()



}

fn CreateCar(car_vector: &mut Vec<Car>){
    clear();

    println!("\nWhat model is your car?");
    
    let model = getInput();


    println!("\nWhat year was it made? ");

    let year = getInput().trim().parse::<u32>().unwrap();
    let new_car = Car{
        model, year
    };

    //Push takes ownership of new_car. 
    car_vector.push(new_car);
    println!("{:#?}", car_vector);
    

}

fn DeleteCar(car_vector: &mut Vec<Car>){


}