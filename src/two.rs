const MUL: i64 = 2;
const ADD: i64 = 1;
const HALT: i64 = 99;

fn main() {
    let program: Vec<i64> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,19,5,23,1,23,5,27,2,27,10,31,1,31,9,35,1,35,5,39,1,6,39,43,2,9,43,47,1,5,47,51,2,6,51,55,1,5,55,59,2,10,59,63,1,63,6,67,2,67,6,71,2,10,71,75,1,6,75,79,2,79,9,83,1,83,5,87,1,87,9,91,1,91,9,95,1,10,95,99,1,99,13,103,2,6,103,107,1,107,5,111,1,6,111,115,1,9,115,119,1,119,9,123,2,123,10,127,1,6,127,131,2,131,13,135,1,13,135,139,1,9,139,143,1,9,143,147,1,147,13,151,1,151,9,155,1,155,13,159,1,6,159,163,1,13,163,167,1,2,167,171,1,171,13,0,99,2,0,14,0];

    println!("Part 1 : {}", run_program(program.clone(), 12, 2));

    for noun in 0..100i64 {
        for verb in 0..100i64 {
            let result = run_program(program.clone(), noun, verb);
            if result == 19690720 {
                println!("Part 2 : noun = {}, verb = {}; 100 * noun + verb = {}", noun, verb, 100 * noun + verb);
            }
        }
    }
}

fn run_program(mut program: Vec<i64>, noun: i64, verb: i64) -> i64 {
    program[1] = noun;
    program[2] = verb;

    let mut pointer = 0;

    while pointer < program.len() {
        let op = program[pointer];
        let pos_a = program[pointer + 1] as usize;
        let pos_b = program[pointer + 2] as usize;
        let pos_c = program[pointer + 3] as usize;
        match op {
            MUL => program[pos_c] =  program[pos_a] * program[pos_b],
            ADD => program[pos_c] =  program[pos_a] + program[pos_b],
            HALT => break,
            _ => {}
        }
        pointer += 4;
    }

    program[0]
}


#[cfg(test)]
mod test {

}
