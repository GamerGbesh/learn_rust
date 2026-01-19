use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    let x = 4;
    
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

struct Cacher<T, V>
where 
    T: Fn(V) -> V,
{
    calculation: T,
    values: HashMap<V, V>,   
}

impl<T, V> Cacher<T, V>
where 
    T: Fn(V) -> V,
    V: Eq + Hash + Clone + std::fmt::Debug,
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher { calculation, values: HashMap::new() }
    }

    fn value(&mut self, arg: V) -> V {
        if let Some(v) = self.values.get(&arg){
            return v.clone();
        }

        let calc = (self.calculation)(arg.clone());
        self.values.insert(arg, calc.clone());
        calc
        
    }

    fn see(&self) {
        println!("{:?}", self.values);
    }
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut cached_result =  Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cached_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cached_result.value(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
            "Today, run for {} minutes!",
            cached_result.value(intensity)
            );
        }
    }
    cached_result.see();
}