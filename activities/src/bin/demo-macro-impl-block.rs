#[derive(Clone, Copy)]
struct Volume(usize);

trait ReagentContainer {
    fn max_volume(&self) -> Volume;
    fn current_volume(&self) -> Volume;
}

struct TallFlask {
    current_volume: Volume,
}

struct TestTube {
    current_volume: Volume,
}

struct Pipette {
    current_volume: Volume,
}

struct OtherTube {
    current_volume: Volume,
    max_volume: Volume,
}

fn main() {}
