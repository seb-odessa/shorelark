use rand::{Rng, RngCore};
use std::iter::once;

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug, PartialEq)]
pub struct Network {
    layers: Vec<Layer>,
}
impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
            .iter()
            .take(layers.len() - 1)
            .zip(layers.iter().skip(1))
            .map(|(input, output)| Layer::random(rng, input.neurons, output.neurons))
            .collect();

        Self { layers }
    }
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }
    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
}

#[derive(Debug, PartialEq)]
struct Layer {
    neurons: Vec<Neuron>,
}
impl Layer {
    fn random(rng: &mut dyn RngCore, input: usize, output: usize) -> Self {
        let neurons = (0..output).map(|_| Neuron::random(rng, input)).collect();
        Self { neurons }
    }

    fn propagate(&self, inputs: &Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(inputs))
            .collect()
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
impl PartialEq for Neuron {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other)
    }
}
impl Neuron {
    fn approx_eq(&self, other: &Self) -> bool {
        abs_diff_eq(&self.bias, &other.bias)
            && self
                .weights
                .iter()
                .zip(other.weights.iter())
                .all(|(this, other)| abs_diff_eq(this, other))
    }

    fn random(rng: &mut dyn RngCore, size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }

    fn propagate(&self, inputs: &Vec<f32>) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        inputs
            .iter()
            .zip(&self.weights)
            .fold(self.bias, |acc, (input, weight)| acc + input * weight)
            .max(0.0)
    }

    fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

fn abs_diff_eq(lhv: &f32, rhv: &f32) -> bool {
    const EPSILON: f32 = 1e-6;
    (lhv - rhv).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    /// https://habr.com/ru/companies/timeweb/articles/818985/
    ///
    ///
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    mod neuron {
        use super::*;

        #[test]
        fn random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);
            let expected = Neuron {
                bias: -0.6255188,
                weights: vec![0.67383957, 0.8181262, 0.26284897, 0.5238807],
            };

            assert_eq!(neuron, expected);
        }

        #[test]
        fn propagate() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            assert_relative_eq!(neuron.propagate(&vec![-10.0, -10.0]), 0.0,);
            assert_relative_eq!(
                neuron.propagate(&vec![0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }
    }

    mod layer {
        use super::*;

        #[test]
        fn random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer: Layer = Layer::random(&mut rng, 2, 1);

            assert_eq!(layer.neurons.len(), 1);
            let expected = Neuron {
                bias: -0.6255188,
                weights: vec![0.67383957, 0.8181262],
            };
            assert!(layer.neurons.into_iter().all(|neutron| neutron == expected));
        }

        #[test]
        fn propagate() {
            let neuron: Neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            let layer = Layer {
                neurons: vec![neuron],
            };

            assert!(layer
                .propagate(&vec![-10.0, -10.0])
                .iter()
                .all(|solution| abs_diff_eq(solution, &0.0)));

            assert!(layer
                .propagate(&vec![0.5, 1.0])
                .iter()
                .all(|solution| abs_diff_eq(solution, &((-0.3 * 0.5) + (0.8 * 1.0) + 0.5))));
        }
    }

    mod network {
        use super::*;

        #[test]
        fn random() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(
                &mut rng,
                &vec![LayerTopology { neurons: 2 }, LayerTopology { neurons: 1 }],
            );

            let expected = Network {
                layers: vec![Layer {
                    neurons: vec![Neuron {
                        bias: -0.6255188,
                        weights: vec![0.67383957, 0.8181262],
                    }],
                }],
            };
            assert_eq!(network, expected);
        }

        #[test]
        fn propagate() {
            let network = Network {
                layers: vec![Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![-0.3, 0.8],
                    }],
                }],
            };

            assert!(network
                .propagate(vec![-10.0, -10.0])
                .iter()
                .all(|solution| abs_diff_eq(solution, &0.0)));

            assert!(network
                .propagate(vec![0.5, 1.0])
                .iter()
                .all(|solution| abs_diff_eq(solution, &((-0.3 * 0.5) + (0.8 * 1.0) + 0.5))));
        }

        #[test]
        fn weights() {
            let network = Network {
                layers: vec![
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.1,
                            weights: vec![0.2, 0.3, 0.4],
                        }],
                    },
                    Layer {
                        neurons: vec![Neuron {
                            bias: 0.5,
                            weights: vec![0.6, 0.7, 0.8],
                        }],
                    },
                ],
            };

            let actual = network.weights().collect::<Vec<_>>();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }

        #[test]
        fn from_weights() {
            let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            assert_relative_eq!(actual.as_slice(), weights.as_slice());
        }
    }
}
