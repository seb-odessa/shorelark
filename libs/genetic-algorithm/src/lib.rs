///
///  Учимся летать: симуляция эволюции на Rust. 3/5
///  https://habr.com/ru/companies/timeweb/articles/820699/
///

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct GeneticAlgorithm;
impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO отбор
                // TODO скрещивание
                // TODO мутация
                todo!()
            })
            .collect()
    }
}
