mod emulator;

fn part1(source: &str) {
    let p = emulator::Program::new(source).expect("failed to parse program");
    let output =
        emulator::Emulator::run_program_with_input(p, vec![1]).expect("failed to run program");
    println!("Part 1 output: {:?}", output);
}

fn part2(source: &str) {
    let p = emulator::Program::new(source).expect("failed to parse program");
    let output =
        emulator::Emulator::run_program_with_input(p, vec![2]).expect("failed to run program");
    println!("Part 2 output: {:?}", output);
}

fn main() {
    let source = include_str!("input.txt");
    part1(source);
    part2(source);
}
