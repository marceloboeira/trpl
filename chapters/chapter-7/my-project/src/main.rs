mod sound;
use rand::Rng;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("{}", secret_number);
}
