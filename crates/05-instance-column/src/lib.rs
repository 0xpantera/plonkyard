//! Homework 2 instance-column challenge.
//!
//! Goal:
//! Prove `a * b * c * d * e * f * g = h` for `K` rows, where the seven
//! factors are private witnesses and each `h` is a public instance value.
//!
//! Introduces:
//! - querying an instance column directly inside a custom gate
//! - keeping public inputs separate from private circuit witnesses
//!
//! PLONKish concepts:
//! - public inputs participating directly in polynomial constraints
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Selector},
    poly::Rotation,
};

#[derive(Clone, Copy, Default)]
struct HighDegreeRow<F: Field> {
    // One logical row of private witness values. `h` is deliberately absent:
    // the verifier supplies it separately through the instance column.
    a: Value<F>,
    b: Value<F>,
    c: Value<F>,
    d: Value<F>,
    e: Value<F>,
    f: Value<F>,
    g: Value<F>,
}

pub struct HighDegreeCircuit<F: Field, const K: usize> {
    // `K` is a compile-time circuit-shape parameter, not a field element or
    // public input. The circuit assigns exactly K private witness rows.
    rows: [HighDegreeRow<F>; K],
}

impl<F: Field, const K: usize> Default for HighDegreeCircuit<F, K> {
    fn default() -> Self {
        Self {
            // Build K rows of unknown Values for key generation. The circuit
            // shape stays fixed while the private witnesses are removed.
            rows: std::array::from_fn(|_| HighDegreeRow::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HighDegreeConfig {
    // These handles must survive into `synthesize`, where the prover assigns
    // the seven private factors into advice columns.
    advice: [Column<Advice>; 7],
    // The selector must also survive so `synthesize` can enable the gate on
    // each assigned row.
    q_mul: Selector,
}

impl<F: Field, const K: usize> Circuit<F> for HighDegreeCircuit<F, K> {
    type Config = HighDegreeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare the circuit's columns and selector.
        let advice = std::array::from_fn(|_| meta.advice_column());
        let instance = meta.instance_column();
        let q_mul = meta.selector();

        // 2. Define the local polynomial constraint.
        meta.create_gate("high_degree", |meta| {
            // Read the seven private factors from advice cells in the current
            // row and the public output from the matching instance row.
            let queried_advice = advice.map(|col| meta.query_advice(col, Rotation::cur()));
            let [a, b, c, d, e, f, g] = queried_advice;
            let h = meta.query_instance(instance, Rotation::cur());
            let q_mul = meta.query_selector(q_mul);

            // When q_mul = 1, this expression must evaluate to zero, enforcing:
            // a * b * c * d * e * f * g = public h.
            vec![q_mul * (a * b * c * d * e * f * g - h)]
        });

        // `instance` does not need to be returned in the config. The gate has
        // already recorded its query, and `synthesize` never assigns public
        // inputs or calls `constrain_instance` in this direct-query design.
        HighDegreeConfig { advice, q_mul }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(
            || "high_degree",
            |mut region| {
                for (offset, row) in self.rows.iter().enumerate() {
                    // Keep the row values in the same order as the advice
                    // columns so they can be assigned together with `zip`.
                    let vals = [row.a, row.b, row.c, row.d, row.e, row.f, row.g];

                    for (column, value) in config.advice.iter().zip(vals) {
                        // Only private advice values are assigned here. Public
                        // instance values are supplied separately to the prover.
                        region.assign_advice(|| "value", *column, offset, || value)?;
                    }

                    // Enable the product constraint for this private/public row.
                    config.q_mul.enable(&mut region, offset)?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::pasta::Fp;

    #[test]
    // One private row and its matching public product satisfy the direct
    // advice-to-instance gate constraint.
    fn valid_witness() {
        let row = HighDegreeRow {
            a: Value::known(Fp::from(1)),
            b: Value::known(Fp::from(2)),
            c: Value::known(Fp::from(3)),
            d: Value::known(Fp::from(4)),
            e: Value::known(Fp::from(5)),
            f: Value::known(Fp::from(6)),
            g: Value::known(Fp::from(7)),
        };

        // The outer vector represents instance columns; the inner vector holds
        // this column's public values by row. Here instance[0][0] is h = 5040.
        let instance = vec![vec![Fp::from(5040)]];

        let k = 4;
        let circuit = HighDegreeCircuit::<Fp, 1> { rows: [row] };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }
}
