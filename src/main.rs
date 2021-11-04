use rand::{
    distributions::{Bernoulli, Distribution},
    thread_rng, Rng,
};
use std::{
    fmt,
    fmt::{Display, Formatter},
    iter::repeat,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PersonKind {
    A,
    B,
    C,
}

impl Display for PersonKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    const A_COUNT: usize = 700;
    const B_COUNT: usize = 200;
    const C_COUNT: usize = 100;

    let rng = thread_rng();
    let mut population = repeat(PersonKind::A)
        .take(A_COUNT)
        .chain(repeat(PersonKind::B).take(B_COUNT))
        .chain(repeat(PersonKind::C).take(C_COUNT))
        .collect::<Vec<_>>();

    assert_eq!(population.len(), 1000);

    let rng_survivors = remove_by_rng(rng, population, 0.5);
    let shuffle_survivors = remove_by_shuffle(rng, population, 900);
}

fn remove_by_rng<R: Rng + ?Sized>(
    rng: &mut R,
    population: Vec<PersonKind>,
    probability: f64,
) -> Vec<PersonKind> {
    let d = Bernoulli::new(probability).unwrap();
    population.into_iter().filter(|p| d.sample(rng)).collect()
}

fn remove_by_shuffle<R: Rng + ?Sized>(rng: &mut R, population: &mut Vec<PersonKind>, count: usize) {
}
