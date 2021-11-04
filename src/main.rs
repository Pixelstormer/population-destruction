use rand::{
    distributions::{Bernoulli, Distribution},
    prelude::SliceRandom,
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

    let mut rng = thread_rng();
    let population = repeat(PersonKind::A)
        .take(A_COUNT)
        .chain(repeat(PersonKind::B).take(B_COUNT))
        .chain(repeat(PersonKind::C).take(C_COUNT))
        .collect::<Vec<_>>();

    assert_eq!(population.len(), 1000);

    let mut rng_pop = population.clone();
    let mut shuffle_pop = population;

    let rng_count = get_count(&rng_pop);
    let shuffle_count = get_count(&shuffle_pop);

    println!(
        "RNG BEFORE: A = {}, B = {}, C = {}, Total = {}",
        rng_count.0,
        rng_count.1,
        rng_count.2,
        rng_pop.len()
    );

    remove_by_rng(&mut rng, &mut rng_pop, 0.9);
    let rng_count = get_count(&rng_pop);

    println!(
        "RNG AFTER: A = {}, B = {}, C = {}, Total = {}",
        rng_count.0,
        rng_count.1,
        rng_count.2,
        rng_pop.len()
    );

    println!(
        "SHUFFLE BEFORE: A = {}, B = {}, C = {}, Total = {}",
        shuffle_count.0,
        shuffle_count.1,
        shuffle_count.2,
        shuffle_pop.len()
    );

    remove_by_shuffle(&mut rng, &mut shuffle_pop, 900);
    let shuffle_count = get_count(&shuffle_pop);

    println!(
        "SHUFFLE AFTER: A = {}, B = {}, C = {}, Total = {}",
        shuffle_count.0,
        shuffle_count.1,
        shuffle_count.2,
        shuffle_pop.len()
    );
}

fn get_count(pop: &[PersonKind]) -> (usize, usize, usize) {
    let (mut a_count, mut b_count, mut c_count) = (0, 0, 0);
    for p in pop {
        match p {
            PersonKind::A => a_count += 1,
            PersonKind::B => b_count += 1,
            PersonKind::C => c_count += 1,
        }
    }
    (a_count, b_count, c_count)
}

fn remove_by_rng<R: Rng + ?Sized>(
    rng: &mut R,
    population: &mut Vec<PersonKind>,
    probability_of_removal: f64,
) {
    let d = Bernoulli::new(1.0 - probability_of_removal).unwrap();
    population.retain(|_| d.sample(rng));
}

fn remove_by_shuffle<R: Rng + ?Sized>(
    rng: &mut R,
    population: &mut Vec<PersonKind>,
    count_to_remove: usize,
) {
    let c = population.len() - count_to_remove;
    let (new_pop, _) = population.partial_shuffle(rng, c);
    *population = new_pop.to_vec();
}
