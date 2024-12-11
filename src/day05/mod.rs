use std::collections::HashMap;

#[inline]
fn parse(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut mapping: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates :  Vec<Vec<i32>> = vec![];

    input
        .lines()
        .for_each(|line|{
            if line.contains("|"){
                let (before, after) = line.split_once("|").expect("should have split");
                let before = before.parse().expect("should have been number");
                let after = after.parse().expect("should have been number");
                let entry : &mut Vec<i32> = mapping.entry(before).or_insert( vec![]);
                entry.push(after);
            }
            if line.contains(","){
                updates.push(line.split(",").map(|part| part.parse().expect("should have been number")).collect());
            }
        });

    (mapping, updates)
}

fn qualify_update(order : &HashMap<i32,Vec::<i32>>, update : &Vec<i32>) -> bool{
    for i in 0..update.len(){
        let current = update[i];
        
        for j in i..update.len(){
            if order.contains_key(&update[j]) && order[&update[j]].contains(&current){
                return false
            }
        }
    }
    true
}

fn fix_update(order : &HashMap<i32, Vec::<i32>>, update : &mut Vec<i32>) -> bool{
    let mut modified = false;
    for i in 0..update.len(){
        let mut current = update[i];
        
        for j in i..update.len(){
            if order.contains_key(&update[j]) && order[&update[j]].contains(&current){
                update.swap(i, j);
                current = update[i];
                modified = true;
            }
        }
    }
    modified
}


#[inline]
pub fn part1(input: &str) -> i32 {
   let (order, mut updates) = parse(input);
   let sum = updates.iter_mut().filter(|update | qualify_update(&order, *update)).map(|update|  update[update.len()/2]).sum();
   sum
}

#[inline]
pub fn part2(input: &str) -> i32{
    let (order, mut updates) = parse(input);
    let sum = updates.iter_mut().filter_map(|update | {
      match fix_update(&order, update){
        true => Some(update[update.len()/2]),
        false => None
      }}).sum();
    sum
}


crate::aoctest!(143, 6267, 123, 5184);