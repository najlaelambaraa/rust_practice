
// struct Person{
//     name: String, //type stocké dans le heap (tas)
//     age: u8, //type stocké dans le stack
// }
// // fonction qui crée une instance de personne
// fn create_person(name: String, age: u8) -> Person {
//     Person{
//         name,
//         age,
//     }
//  }
//  fn celebrate_birthday(person: &mut Person) {
//     //&mut Person est une référence mutable à une instance de Person permet de modifier l'instance
//      person.age += 1;
//  }
use std::thread; 
use std::sync::{Arc, Mutex};
fn main() {
    // let ma_valeur_virgule: f64 = 0.3;
    // let valeur_entiere: usize = 42;
    // let valeur_entiere_type: i32 = 42;
    // let mon_usize: usize = 5usize;
    // let petite_donnee: i8 = 55;
    // let mut somme= mon_usize + valeur_entiere_type as usize;
    // println!("somme: {}", somme);
    // somme += 42;
    // println!("somme: {}", somme);
    
    // println!("Hello, world!");

    //Compiler avec UTF-8
    // let ma_str = "ma str";
    // let ma_string = String::from("ma string");
    // let mut ma_format:String = format!("ceci est une string avec {ma_str} et {ma_string}");
    // println!("{}", ma_format);
    // ma_format.push_str("??");
    // println!("{}", ma_format);
    // ma_format.push('!');
    // let ma_string_format_str: &str = ma_string.as_str();
    // println!("{}", ma_string_format_str);
    // let mon_caractere: char = ma_string.chars().nth(3).unwrap();
    // println!("Mon caractere :{}", mon_caractere);

    // for lettre in ma_string_format_str.chars() {
    //     println!("{}", lettre);
    // }
    // let name = String::from("Alice");
    // //let name: &str = "Alice";
    // let age = 30;
    // let mut p1 = create_person(name, age);
    // p1.age = 31;
    // p1.name = String::from("Bob");
    // celebrate_birthday(&mut p1);
    // println!("{} is {}", p1.name, p1.age);
   
    let v: Vec<i32> = Vec::new();
    let mut v = vec![];

    // match find_max(&v) {
    //     Ok(max) => println!("Le maximum est : {}", max),
    //     Err(e) => println!("Erreur : {}", e),
    // }
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
// fn print_even_numbers(v: &[i32]) {
//     for i in v.iter() {
//         if i % 2 == 0 {
//             println!("{}", i);
//         }
//     }
// }
// fn find_max(numbers: &[i32]) -> Result<i32, String> {
//     if numbers.is_empty() {
//         return Err("The list of numbers is empty".to_string());
//     }

//     let mut max = numbers[0];
//     for &i in numbers.iter() {
//         if i > max {
//             max = i;
//         }
//     }
//     Ok(max)
// }

