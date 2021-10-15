
use std::collections::hash_map::Entry;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
fn main() {
    //closures start with a pair of vertical pipes ”|”, inside which we specify the parameters of the closure.
    //let contains the DEFINITION of an anonymous function, not the result
    let mut expensive_closure = Cacher::new(|num| {
            println!("expensive calculation... result: ");
            thread::sleep(Duration::from_secs(2));
            num*2
        });
    println!("Calling closure, {} ", expensive_closure.value(1)); 
    println!("Calling closure, {} ", expensive_closure.value(2)); 
    println!("Calling closure, {} ", expensive_closure.value(3)); 
    println!("Calling closure, {} ", expensive_closure.value(1)); 
    println!("Calling closure, {} ", expensive_closure.value(2));  
    println!("Calling closure, {} ", expensive_closure.value(3)); //value is in cache, do not performs expensive calculation
    println!("HashMap {:#?}",expensive_closure.cache);
}

// Private struct implementing closure with parameter and return type u32.
// This is private because we want cacher to manage the struct field rather than code.
#[derive(Debug)]
struct Cacher<T>
where T: Fn(u32) -> u32,		//each closure instance has a unique type signature
{
    calculation: T,
    cache: HashMap<u32, u32>,
}

impl<T> Cacher <T>
where T: Fn(u32) -> u32,	
{
//Cacher::new returns a Cacher instance that holds calculation-closure
    fn new(calculation: T) -> Cacher<T> { 	 
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }
//we call value, whenever we need a result of expensive calculation-closure, it will store if is not in the memory or retrieve the value if it was calculated before.   
    fn value(&mut self, arg: u32) -> u32 {
        let previously_calculated= self.cache.entry(arg);  
        match previously_calculated {
            Entry::Occupied(v) => *v.get(), //retrieve previous value and skip expensive_calculation
            _ =>{ 
                let value = (self.calculation)(arg);        //call expensive_closure
                self.cache.insert(arg, value);              //insert new entry
                value
            }
        }
    }
}
