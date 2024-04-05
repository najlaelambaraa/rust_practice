use std::thread; 
use std::sync::{Arc, Mutex};
fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![];

        let counter: Arc::<Mutex::<i32>> = Arc::new(Mutex::new(0));
        let mut handles= vec![];
        for _ in 0..3 {
            let counter = Arc::clone(&counter);//cloner l'arc, argumentant le compteur de références pour pouvoir partager avec un thread
            let handle = thread::spawn(move || { //demarer un nouveau thread
                let mut num = counter.lock().unwrap(); //verrouille le Mutex avant de l'incrémenter
                *num += 1;
            });
            handles.push(handle);
        } 
        for handle in handles {
            handle.join().unwrap(); //attendre la fin de chaque threads se terminent
        }  
        println!("Result: {}", *counter.lock().unwrap());  //Affihce la valeur final en verrouillant le Mutex
     

}

fn print_even_numbers(v: &[i32]) {
    for i in v.iter() {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}


