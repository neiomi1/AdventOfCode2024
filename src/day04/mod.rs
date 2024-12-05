use std::collections::HashMap;


#[inline]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    find_matches(input, "XMAS", &[(-1,1), (1,1), (1,0), (0,1)].to_vec()).len() as i32
}


#[inline]
pub fn part2(input: &str) -> i32{
    let all_matches = find_matches(input, "MAS", &[(-1,1), (1,1)].to_vec());
    // println!("matches: {:?}", all_matches);
    let filtered : Vec<Vec<(usize, usize)>> =
    all_matches
    .clone()
    .into_iter()
    .filter(|word| 
        all_matches.iter().any(|m| m[1] == word[1] && m[0] != word[0])
    ).collect();
    // println!("filtered: {:?}", filtered);
    let unique_xs: std::collections::HashSet<_> = filtered.iter().map(|word| &word[1]).copied().collect();
    // println!("found: {:?}", unique_xs);
    unique_xs.len() as i32
}


#[inline]
pub fn find_matches(input: &str, search_word : &str, directions : &Vec<(isize, isize)>) -> Vec<Vec<(usize, usize)>> {
    let search_word_characters : Vec<char> = search_word.chars().collect();
    let mut search_word_characters_reversed = search_word_characters.clone();
    search_word_characters_reversed.reverse();

    let parsed = parse(input);
    
    let mut potential_matches = HashMap::new();
    for direction in directions{
        let x : Vec<Vec<(usize, usize)>> = vec![];
        potential_matches.insert(*direction, x);
    }

    let mut complete: Vec<Vec<(usize, usize)>> = vec![];

    for (y, line) in parsed.iter().enumerate(){
        for (x, char) in line.iter().enumerate(){
            for direction in directions{
                complete.append(&mut check_matches(potential_matches.get_mut(direction).expect("expected entry for direction"), x, y, *direction, &parsed, &search_word_characters, char, &search_word_characters_reversed));
           
                if *char == search_word_characters[0] || *char == search_word_characters_reversed[0] {
                    let bound_x = x as isize + direction.0 * search_word_characters.len() as isize;
                    let bound_y = y as isize + direction.1 * search_word_characters.len() as isize;
                    if -1 <= bound_x && bound_x <= line.len() as isize && -1 <= bound_y && bound_y <= parsed.len() as isize{
                        potential_matches.get_mut(direction).expect("expected entry for direction").push(vec![(x,y)]);
                    }
                }
            }
        }
    }

    complete
}

fn check_matches(potential_matches: &mut Vec<Vec<(usize, usize)>>, x: usize, y: usize, direction : (isize, isize), parsed: &Vec<Vec<char>>, search_word_characters: &Vec<char>, char: &char, search_word_characters_reversed: &Vec<char>) -> Vec<Vec<(usize, usize)>> {
    let mut completed = vec![];
    // println!("potential matches: {:?}, direction: {:?}", potential_matches, direction);
    let mut unchecked = vec![];
    potential_matches.retain(|potential_match| {
        let last_position = potential_match.last().unwrap();
        let expected_position = (last_position.0.checked_add_signed(direction.0).unwrap() , last_position.1.checked_add_signed(direction.1).unwrap()); 
        if expected_position.0 != x || expected_position.1 != y{
            // println!("can't check {x},{y} for direction direction: {:?}", direction);
            unchecked.push(potential_match.clone());
            return false;
        } 

        if check_next_character(parsed, potential_match, search_word_characters, char) || 
          check_next_character(parsed, potential_match, search_word_characters_reversed, char){
            if potential_match.len()+1 == search_word_characters.len(){
                let mut completed_clone = potential_match.clone();
                completed_clone.push((x,y));
                // println!("completed: {:?} with {char}", completed_clone);
                completed.push(completed_clone);
                return false;
            }
            return true;
        }
    
        return false;
    });

    // println!("retained {:?}", potential_matches);
    potential_matches.iter_mut().for_each(|potential_match| potential_match.push((x,y)));
    potential_matches.append(&mut unchecked);

    completed
}

fn check_next_character(parsed : &Vec<Vec<char>>, potential_match : &Vec<(usize, usize)>, search_word: &Vec<char>, next_character: &char) -> bool {
    for (i, pos) in potential_match.iter().enumerate(){
        if parsed[pos.1][pos.0] != search_word[i]{ return false;}
    }
    *next_character == search_word[potential_match.len()]
}

crate::aoctest!(18, 2573, 9, 1850);