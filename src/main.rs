
// fn main() {
//     let number = 7;

//     // the if condition must be a boolean
//     // Unlike Javascript, Rust will not try to convert non-boolean types to a boolean
//     if number != 0 {
//         println!("number was something other than zero");
//     }
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// Rust checks for the first condition that is true in the if block, and then it executes the code associated with that condition, if there is one and ignores the rest of the conditions.
// fn main(){
//     let number = 6;
//     if number % 4 == 0 {
//         println!("number is divisible by 4");

//     }
//     else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     }
//     else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     }
//     else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main(){
//     let condition = true;
//     // Rust must know at compile time what type of value number will be.
//     // if the else statement has 'six' as a string, the compiler will throw an error.
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// One of the uses of a loop is to rety an operation you know might fail, such as checking whether a thread has completed its job.
// fn main() {
//   let mut counter = 0;

//   let result = loop {
//     counter +=1;

//     if counter == 10 {
//       break counter * 2;
//     }
//   };
//     println!("The result is {result}");
// }

// Loop Labels are to disambiguate which loop we want to break or continue when dealing with nested loops.
// The following code will exit the outer loop when count reaches 2. 
// The first break without a label will exit the inner loop and the second break with a label will exit the outer loop.
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaning = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -=1;
//         }
//         count +=1; 
//     }
//     println!("End count = {count}");
// }


// Conditional loops with while
// fn main() {
//     let mut number = 3;

//     // this construct elimates the need for lots of nesting with loop, if, else, and break.
//     while number !=0 {
//         println!("{number}");
//         number -=1;
//     }
//     println!("LIFTOFF!!!");
// }

// Looping through a collection with for
fn main(){
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // the compiler adds runtime code to perform the check that the index value stays within the bounds of the array.
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index +=1;
    // }

    // As an alternative, we can use a for loop and execute some code for each item in a collection.
    // The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

    //Using reverse method to iterate over the elements of a collection in reverse order.
    for number in (1..4).rev(){ // 1..4 is a range that includes 1, 2, and 3 but not 4, and rev reverses the range.
        println!("{number}!");
    }
    println!("LIFTOFF!!!")
}