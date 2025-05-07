fn main() {
    // Create an array and access elements
    let numbers = [1, 2, 3, 4, 5];
    println!("The first number is : {}", numbers[0]);

    // Change array values
    let mut numbers2 = [1, 2, 3, 4, 5, 6];
    numbers2[0] = 100;
    println!("The new first number is : {}", numbers2[0]);

    // Array length
    println!("The first array has {} elements", numbers.len());
    println!("The second array has {} elements", numbers2.len());

    // Print the entire array
    println!("{:?}", numbers);
    println!("{:?}", numbers2);

    println!();

    // Loop through an array
    let fruits = ["apple", "banana", "orange"];
    for fruit in fruits {
        println!("I like {fruit}");
    }

    // Array vs Vector
    let mut cars = ["Volvo", "BMW", "Ford"];
    cars[2] = "Cadillac";
    // cars[3] = "Mazda"; // Error: index out of bounds
    println!("{:?}", cars);

    let mut autos = vec!["Tesla", "Mercedes", "Nissan"];
    autos.push("Porsche");
    println!("{:?}", autos);
}
