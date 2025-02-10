use std::thread;
use std::thread::JoinHandle;

fn main() {
    let hash = 08_810_345_129; // a random hash/number
    let total_possibilities = 12_000_000_000;//the hashed number could be upto 12 billion
    let cup_threads = 12;
    let mut starting_points: Vec<u64> = Vec::new();
    let work = total_possibilities/cup_threads;
    
    for i in 0..cup_threads {
        let starting_point = work*i;
        starting_points.push(starting_point);
    }

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in starting_points.into_iter() {
        let handle = thread::spawn(move || {
            let end = i+work;
            for j in i..end {
                if j == hash {
                    println!("we got him it's: {j}");
                    break;
                }
            }
        });
        handles.push(handle); 
    }
    
    for handle in handles {
        handle.join().unwrap()
    }
}
