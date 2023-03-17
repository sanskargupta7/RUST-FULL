//CLOSURES - anonymous functions you can save in a variable or pass as arguments to other functions
//unlike functions closures can capture values from scope in which they are defined
//closures dont usually require us to annotate the types of input parameters nor the type of return value like functions

//closures are not exactly functions ...they can be passed around as parameters, variables...or even functions

//even though we are allowed to not specify data types for input parameters...its also NOT ALLOWED to have closures work on parameters with different datatypes in same program
//i.e. the first type passed in the closure as input will become the concrete type of the closure

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red, 
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())      //we used most_stocked() method inside the giveaway method ...hence it is a closure
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

}




//instead of calling closures again and again....it is better that we put the input and the losure applied on it inside a struct
//but for doing this we need to use generics


//we can also bind a variable to a closure definition and then we can later call the closure using the variable name
// let list = vec![1, 2, 3];
// println!("Before defining closure: {:?}", list);

// let only_borrows = || println!("From closure: {:?}", list);

// println!("Before calling closure: {:?}", list);
// only_borrows();
// println!("After calling closure: {:?}", list);


//Output
// Before defining closure: [1, 2, 3]
// Before calling closure: [1, 2, 3]
// From closure: [1, 2, 3]
// After calling closure: [1, 2, 3]

//----------------------------------------------------

//now we can take a mutable closure so can it can perform update and change operations

// let mut list = vec![1, 2, 3];
// println!("Before defining closure: {:?}", list);

// let mut borrows_mutably = || list.push(7);

// borrows_mutably();
// println!("After calling closure: {:?}", list);

//OUTPUT
// Before defining closure: [1, 2, 3]
// After calling closure: [1, 2, 3, 7]

//--------------------------------------------------------

//if we want our closure to take the ownership even though its not required....we can use the 'move' keyword
//This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.


// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     thread::spawn(move || println!("From thread: {:?}", list))
//         .join()
//         .unwrap();
// }

//here we spawn a new thread, to give it a closure...
///closure body prints

//now we cannot use 'list' to print or call our list because the 'list' name has gone out of scope after the closure took ownership of our list with the move method.


//--------------------------------------------------------------------

//after the closure has taken the ownership of a value:
/// the body of the closure decides what to do of the borrowed value
/// 1 - move the captured value out of closure
/// 2 - mutate the captured value
/// 3 - neither move or mutate and do nothing
/// 
/// 
/// 

///the way closures captures and handles values affects which traits the closure implements....
/// the traits which closures impplements:
/// 1 - FnOnce - applies to closures that can be called once. a closure that moves captured values OUT OF THE BODY will only implement FnOnce and none of the other Fn traits, because it can be only called once
/// 2 - FnMut - applies to closures which mutate the captured values instead of making them move out...might be called more than once
/// 3 - Fn - capture and mutate nothing from or to the environment...can be called more than once



//lets say we have a program which has a rectangle and orders the list acoording to the width
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{:#?}", list);
// }

///now in sort_by_key() method .....we give an r as parameter and it will have to return the result according to r.width
/// and it calls FnMut closure mutliple times...once for each item in slice
/// 
/// 
/// ///lets say we have another implementATION OF THE CLOSURE IN THE sort_by_key() method

// let mut sort_operations = vec![];
// let value = String::from("by key called");

// list.sort_by_key(|r| {
//     sort_operations.push(value);
//     r.width
// });
// println!("{:#?}", list);

///here we are seeing that we dont know how many times FnMut is being called by the sort_by_key() function 
/// hence here we try to count it by adding a value String from the closure's environment in a specific vector.....IT DOESNT WORK
//DOESNT WORK - because - the ownership of the String gets moved from 'value' to the sort_operations.push() method and the 'value' will go out of scope
//now this tells us that this closure could only be called once because after second time...the value will get moved out of scope

//hence this closure implements FnOnce even though without the ownership operations this would run FnMut mutiple times

//hence to correct this we need to avoid moving the value inside the method()
//instead of doing the above, we can simply take in a counter and increase it by one at each step


list.sort_by_key(|r| {
    num_sort_operations += 1;  /// we can do this and so the problem of copying value inside the closure environment would be avoided
    r.width
});


//----------------------------------------------------------------------------

//ITERATORS

//helps us in iterating over each item in a groupp of items

//In rust iterators dont run until we call methods that consume the iterator to use it

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
}


//this code runs but doesnt do anything

//now we an use these in a number of ways

//in for loop
for val in v1_iter {
    println!("Got: {}", val);
}


//Iterators handle all that logic for you, cutting down on repetitive code you could potentially mess up. 
//Iterators give you more flexibility to use the same logic with many different kinds of sequences, 
//not just data structures you can index into, like vectors.


//iterator implements a trait named Iterator that is defined in the standard library.

//we can also go along an Iterator using next() method for iterators

//next() method is declared in Iterator trait as:
//fn next(&mut self) -> Option<Self::Item>;

//hence we see that it returns Option enum
///if there is a value on iter.next()....we get Some(&value);
/// else we can get None as the result
/// for exampple lets see a test function
/// 
/// 
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();        //we made the iterator mutable because using the next() method changes internal state of the iterator

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

//we made the iterator mutable here as we are using next() method....
//we didnt need to make iterator mutable while using 'for' loop becuase 'for' loop took ownershipp of the iterator and made it mutable behind the scenes

//the values we get from the calls to next() are immutable refernces to values in the vector as we see in the test function

//IF WE WANT THE ITERATOR TO TAKE OWNERSHIPP OF Vector v1 and return owned values....we can call into_iter() method instead of iter()...
//IF WE WANT TO ITERATE OVER MUTABLE REFERENCES, we can call iter_mut() method instead of iter() method

//---------------------------------------------------------------

//METHODS THAT CONSUME ITERATOR

//iterator has a method called sum().....
//this method takes ownershipp of the iterator because it iterates through the items by repeatedly calling next() method hence consuming the iterator.


//------------------------------------------------

//METHODS THAT PRODUCE ITERATORS

//map() method
///
/// takes a closure call on each item as items are iterated through...
/// this method returns a new iterator that produces the modified items

let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);       //using CLOSURE in iterator

//this will produce an iterator which will produce items from the vector after incresing them all by 1
//BUT it will do nothing unless it is used...BECAUSE IT IS LAZYYYY...
//it will show this as a warning that it is doing nothing.....

//though we can use collect() method to get all the data inside the iterator and store it inside a collection


//------------------------------------------------------------

//USING CLoSURES THAT CAPTURE THEIR ENVIRONMENT


//filter() method - filter the data according to some criteria...
//we can have CLOSURE as an ARGUMENT inside filter() method to filter out the data....


//_------------------------------------------------------

//ITERATORS VS LOOPS

//iterators have slightly better performance than for loops
//Hene we can use the iterator in cases where minute improvements in speed is necessary..
//example....- audio decoders



