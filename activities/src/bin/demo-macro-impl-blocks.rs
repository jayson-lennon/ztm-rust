#[derive(Clone, Copy)]
struct Volume(usize);

trait ReagentContainer {
    fn max_volume(&self) -> Volume;
    fn current_volume(&self) -> Volume;
}

macro_rules! impl_reagent_container {
    ($container:ty : $volume:literal) => {
        impl ReagentContainer for $container {
            fn max_volume(&self) -> Volume {
                Volume($volume)
            }
            fn current_volume(&self) -> Volume {
                self.current_volume
            }
        }
    };
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

impl ReagentContainer for OtherTube {
    fn max_volume(&self) -> Volume {
        self.max_volume
    }
    fn current_volume(&self) -> Volume {
        self.current_volume
    }
}

impl_reagent_container!(TallFlask: 32);
impl_reagent_container!(TestTube: 10);
impl_reagent_container!(Pipette: 4);

fn main() {}
