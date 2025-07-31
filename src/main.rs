fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[0..2]
}
fn main() { // Class 335 and 336 lifetimes
    let cities = vec![
        String::from("New York"),
        String::from("Los Angeles"),
        String::from("Chicago"),
    ];
    let two_cities = {
        let cities_reference = &cities;
        select_first_two_elements(cities_reference)
    };
    println!("{:?}", two_cities);

    {
        let coffees = [String::from("Latte"), String::from("mocha")];
        let two_coffees = select_first_two_elements(&coffes);
        println!("{:?}", two_coffees);


    }

}
/*
fn main() { // Class 329 lifetimes
    let dog = String::from("Watson");

    {
        let my_pet = &dog;
        println!("{}", my_pet);
    }
println!("{}", dog);

{
    let my_pet = &dog;
    println!("{}", my_pet);
}

}
*/