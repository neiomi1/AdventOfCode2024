use itertools::Itertools;
use std::vec;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Block{
    id: u64,
    start_index: u64,
    stop_index: u64
}

impl Block {
    fn size(&self) -> u64{
        return self.stop_index - self.start_index
    }

    fn set_index(&mut self, new_index: u64){
        let size = self.size();
        self.start_index = new_index;
        self.stop_index = new_index + size;
    }

    fn split(&mut self, new_size : u64) -> Block{
        let size = self.size();
        self.stop_index = self.start_index + new_size;
        Block { id: self.id, start_index: self.stop_index, stop_index: self.stop_index + size - self.size() }
    }
}

#[inline]
fn parse(input: &str) -> Vec<Block> {
    let mut file_blocks = vec![];

    let mut i = 0;
    let mut is_file  = true;
    let mut file_index = 0;
    for file in input.lines().next().unwrap().chars(){
        let size = file.to_digit(10).unwrap() as u64;
        let end = i + size;
        if is_file{
            file_blocks.push(Block{id: file_index as u64, start_index: i, stop_index: end});
            file_index += 1
        }
        is_file = is_file ^ true;
        i = end;
    }

    file_blocks
}

fn format_disk_fragmentation(files : &Vec<Block>) -> Vec<Block>{
   let mut result = vec![];

   let mut to_insert = files.iter().copied().collect_vec();

   result.push(to_insert[0]);
   to_insert.remove(0);
 
   while let Some(mut file) = to_insert.pop(){
        let free_start = result.last().unwrap().stop_index;
        let mut free_stop = file.start_index;
        if to_insert.len() > 0{
            free_stop = to_insert[0].start_index;
        }
        let free_space = free_stop - free_start;

        if file.size() > free_space{
            let end_split = file.split(free_space);
            to_insert.push(end_split);
        }

        file.set_index(free_start);
        result.push(file);
        
        if file.size() >= free_space{
            result.push(to_insert[0]);
            to_insert.remove(0);
        }
   }

   result
}

fn format_disk_no_fragmentation(files : &Vec<Block>) -> Vec<Block>{
    // let disk_length = &files.last().unwrap().stop_index;
    let mut formatted_disk = files.clone();
    let mut i = formatted_disk.len()-1;
    loop{
        let (file_index, file) = formatted_disk.iter().find_position(|block| block.id == i as u64).unwrap();
        for j in 1..file_index+1{
            let free_start =  formatted_disk[j-1].stop_index;
            let free_stop =  formatted_disk[j].start_index;
            let free_space = free_stop - free_start;
         
            // println!("found free space: {free_space} with ({free_start}, {free_stop}), need {} for {}", file.size(), i);
            if free_space >= file.size(){
                let mut file = file.clone();
                file.set_index(free_start);
                formatted_disk.remove(file_index);
                formatted_disk.insert(j, file);
                break;
            }
        }
        if i == 0{
            break;
        }
        i -= 1;
    }
    formatted_disk
}

#[allow(dead_code)]
fn print_disk(files : &Vec<Block>, length : &u64){
    let mut i = 0;
    for block in files{
        for _ in i..block.start_index{
            print!(".");
        }
        for _ in 0..block.size(){
            print!("{}", block.id);
        }
        i = block.stop_index;
    }
    for _ in i..*length{
        print!(".");
    }
    print!("\n");
}

fn checksum(files : &Vec<Block>) -> u64{
    let mut checksum = 0;
    for file in files{
        let mut block_id = file.start_index;
        for _ in 0..file.size(){
            checksum += block_id * file.id;
            block_id += 1;
        }
    }

    checksum
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let disk = parse(input);
    // let disk_length = &disk.last().unwrap().stop_index;
    // print_disk(&disk, disk_length);

    let formatted_disk = format_disk_fragmentation(&disk);
    // print_disk(&formatted_disk, disk_length);

    checksum(&formatted_disk)
}

#[inline]
pub fn part2(input: &str) -> u64{
    let disk = parse(input);
    // let disk_length = &disk.last().unwrap().stop_index;
    // print_disk(&disk, disk_length);

    let formatted_disk = format_disk_no_fragmentation(&disk);
    // print_disk(&formatted_disk, disk_length);

    checksum(&formatted_disk)
}

crate::aoctest!(1928, 6401092019345, 2858, 6431472344710);