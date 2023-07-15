use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&String, i32> = HashMap::new();

    /* map.insert("Bega".to_string(), 0);
     map.insert("Julia".to_string(), 1);
     map.insert("Berat".to_string(), 2);
     map.insert("Denis".to_string(), 3);
     println!("{:?}",map);*/

    let n1 = "Bega".to_string();
    let n2 = "Julia".to_string();
    let n3 = "Berat".to_string();
    let n4 = "Vlad".to_string();
    map.insert(&n1, 0);
    map.insert(&n2, 1);
    map.insert(&n3, 2);


    map.entry(&n4).or_insert(9);//burada biz hashmap icinde girdigimiz verilere sahip birilerinin olup olmadigini kontrol ediyor

    println!("{:?}",map);

    //println!("{:?}",map[&n1]);

    /*match map.get(&n1) {
        None => {
            println!("Element doesn't exist!");
        }
        Some(mark) => {
            println!("Bega is {}", mark);
        }
    }*/

    /*for (name, mark) in &map {
        println!("{} has {}",name,mark);
    }*/
}
