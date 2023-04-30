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

    println!("Exercise 5 is correct");
}
