use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const N: u64 = 80000000;
    let issues_fav_a = Arc::new(Mutex::new(0));
    let issues_fav_b = Arc::new(Mutex::new(0));
    let paquet: Vec<u32> = vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8,
        8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13,
    ];
    println!("Le programme joue {} millions de parties.", N / 1000000);
    let mut handles = vec![];

    for _ in 0..8 {
        const M: u64 = N / 8;
        let mut paquet = paquet.clone();
        let (issues_fav_a, issues_fav_b) = (Arc::clone(&issues_fav_a), Arc::clone(&issues_fav_b));
        let handle = thread::spawn(move || {
            for _ in 0..M {
                let mut rng = thread_rng();
                paquet.shuffle(&mut rng);
                let paquet_j_a = &paquet[..26];
                // let paquet_j_a = [
                //     14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
                //     14, 14, 14, 14, 14, 14,
                // ];
                let paquet_j_b = &paquet[26..];
                // let paquet_j_b = [
                //     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                // ];
                let premier_tour: u32 = match paquet_j_a[0].cmp(&paquet_j_b[0]) {
                    Ordering::Greater => 1,
                    Ordering::Less => 2,
                    Ordering::Equal => 0,
                };
                let mut i = 1;
                while (paquet_j_a[i] > paquet_j_b[i] && premier_tour == 1)
                    || (paquet_j_a[i] < paquet_j_b[i] && premier_tour == 2)
                {
                    if i == 25 {
                        if premier_tour == 1 {
                            let mut issues_fav_a = issues_fav_a.lock().unwrap();
                            *issues_fav_a += 1;
                        } else {
                            let mut issues_fav_b = issues_fav_b.lock().unwrap();
                            *issues_fav_b += 1;
                        }
                        break;
                    }
                    i += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let issues_fav_a = issues_fav_a.lock().unwrap();
    let issues_fav_b = issues_fav_b.lock().unwrap();
    let issues_fav_tot = *issues_fav_a + *issues_fav_b;

    println!(
        "{} parties ont été gagnées en 26 coups dont: {} par le joueur A et {} par le joueur B.",
        issues_fav_tot, *issues_fav_a, *issues_fav_b
    );
}
