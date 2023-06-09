use rand::Rng;

fn main() {
    // Exercise #1
    // insert code below: 
    let on_mars_right_now: bool = false;
    // insert code above
    
    assert_eq!(on_mars_right_now, false, "If you see a panic, be sure to create the variable and assign it a value.");
    println!("Exercise 1 is correct.");

    // Exercise #2
    // insert code below:
    let fruits = ["mango", "banana", "guava", "kiwi", "strawberry"];
    // insert code above

    assert_eq!(fruits, ["mango", "banana", "guava", "kiwi", "strawberry"], "If you see an assertion error, ensure the variable contains all the strings in the provided order");
    println!("Exercise 2 is correct.");

    // Exercise #3
    // insert code below: 
    let vegetables = ["eggplant", "broccoli", "carrot", "cauliflower", "zucchini"];
    // insert code above

    assert_eq!(vegetables, ["eggplant", "broccoli", "carrot", "cauliflower", "zucchini"], "If you see an assertion error, ensure the variable contains all the strings in the provided order");
    println!("Exercise 3 is correct.");
    
    // Exercise #4
    // insert code below: 
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // insert code above
   
    assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "If you see an assertion error, ensure the variable contains the numbers 1-10 in order.");
    println!("Exercise 4 is correct.");
    
    // Exercise #5
    // insert code below:
    let mut fruits = vec!["mango", "banana", "guava", "kiwi", "strawberry"];

    fruits.push("tomato");

    assert_eq!(
        fruits,
        vec!["mango", "banana", "guava", "kiwi", "strawberry", "tomato"],
        "Ensure the variable contains all the strings in the right order"
    );

    println!("Exercise 5 is correct.");
    
    // Exercise #6
    // insert code below:
    let mut vegetables = vec!["eggplant", "broccoli", "carrot", "cauliflower", "zucchini"];

    vegetables.push("tomato");

    assert_eq!(vegetables, ["eggplant", "broccoli", "carrot", "cauliflower", "zucchini", "tomato"]);
    println!("Exercise 6 is correct.");
    // insert code above

    // Exercise #7
    // insert code below:
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    numbers.reverse();

    assert_eq!(numbers, [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    println!("Exercise 7 is correct.");
    // insert code above
    
    // Exercise #8
    // insert code below:
    let mut vegetables = vec!["eggplant", "broccoli", "carrot", "cauliflower", "zucchini", "tomato"];

    vegetables.sort();

    assert_eq!(vegetables, ["broccoli", "carrot", "cauliflower", "eggplant", "tomato", "zucchini"]);
    println!("Exercise 8 is correct.");
    // insert code above

    
    // Exercise #9
    // insert code below:
    let mut fruits = vec!["banana", "kiwi", "mango", "strawberry", "guava", "tomato"];

    fruits.sort_by(|a, b| b.cmp(a));

    assert_eq!(fruits, ["tomato", "strawberry", "mango", "kiwi", "guava", "banana"]);
    println!("Exercise 9 is correct.");
    // insert code above

    
    // Exercise #10
    // insert code below:
    let mut fruits = vec!["banana", "kiwi", "mango", "strawberry", "guava", "tomato"];
    let mut vegetables = vec!["eggplant", "broccoli", "carrot", "cauliflower", "zucchini", "tomato"];

    fruits.sort_by(|a, b| b.cmp(a));
    vegetables.sort();

    let mut fruits_and_veggies = fruits;
    fruits_and_veggies.extend(&vegetables);

    assert_eq!(fruits_and_veggies, ["tomato", "strawberry", "mango", "kiwi", "guava", "banana", "broccoli", "carrot", "cauliflower", "eggplant", "tomato", "zucchini"]);
    println!("Exercise 10 is correct.");
    // insert code above

    //#Basic Functions
    //> ### Hint Be sure to return values from your function definitions. The assert statements will call your function(s) for you.

    // Exercise #11
    // insert code below:
    let mut rng = rand::thread_rng();
    let positive_even_number = rng.gen_range(2, 102) / 2 * 2;
    let negative_even_number = rng.gen_range(-100, 0) / 2 * 2;
    let positive_odd_number = rng.gen_range(1, 100) / 2 * 2 + 1;
    let negative_odd_number = rng.gen_range(-101, -1) / 2 * 2 + 1;

    println!("We now have some random numbers available for future exercises.");
    println!("The random positive even number is {}", positive_even_number);
    println!("The random positive odd number is {}", positive_odd_number);
    println!("The random negative even number is {}", negative_even_number);
    println!("The random negative odd number is {}", negative_odd_number);
    // insert code above
}
