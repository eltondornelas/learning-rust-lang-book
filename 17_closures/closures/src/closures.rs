use std::thread;
use std::time::Duration;

pub struct Cacher<T> where T: Fn(u32) -> u32,
{
    pub calculation: T,
    pub value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// 4th scenario: using struct with closure
pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    // assert_eq!(v2, 2); // error
    assert_eq!(v2, 1); // ok
}


// 3rd scenario: using closure to store code
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//
//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }

// on the 3rd scenario there is no need for this method anymore
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// 2nd scenario: refactoring using functions
// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = simulated_expensive_calculation(intensity);
//
//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result);
//         println!("Next, do {} situps!", expensive_result);
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_result);
//         }
//     }
// }

// 1st scenario: without functions
// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }