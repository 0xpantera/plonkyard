//! Placeholder for Homework 2 Problem 3.
//!
//! Goal: enforce `a * b * c * d * e * f * g = h` for `K` rows.
//! `K` is a Rust const generic here: it controls how many private witness rows
//! the circuit assigns and constrains.
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Selector},
    poly::Rotation,
};

#[derive(Clone, Copy, Default)]
struct HighDegreeRow<F: Field> {
    // One logical row of private witness values.
    // The first seven values are multiplied together; `h` is the claimed output.
    a: Value<F>,
    b: Value<F>,
    c: Value<F>,
    d: Value<F>,
    e: Value<F>,
    f: Value<F>,
    g: Value<F>,
    h: Value<F>,
}

pub struct HighDegreeCircuit<F: Field, const K: usize> {
    // This exercise constrains exactly K rows. K is part of the circuit type,
    // not a public input or private field element assigned in the circuit.
    rows: [HighDegreeRow<F>; K],
}

impl<F: Field, const K: usize> Default for HighDegreeCircuit<F, K> {
    fn default() -> Self {
        Self {
            // `std::array::from_fn` builds an array for generic K. Deriving
            // Default for `[HighDegreeRow<F>; K]` does not work for arbitrary K.
            rows: std::array::from_fn(|_| HighDegreeRow::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HighDegreeConfig {
    // These eight advice columns are homogeneous, so an array keeps the config
    // compact. advice[0..=6] are multiplied together; advice[7] is h.
    advice: [Column<Advice>; 8],
    // Selectors turn a gate on for specific rows.
    q_mul: Selector,
}

impl<F: Field, const K: usize> Circuit<F> for HighDegreeCircuit<F, K> {
    type Config = HighDegreeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare global circuit structure / column capabilities.
        let advice = std::array::from_fn(|_| meta.advice_column());
        // A simple selector is enough because it only turns this one product
        // constraint on or off for each row.
        let q_mul = meta.selector();

        // 2. Define local algebraic constraints.
        meta.create_gate("high_degree", |meta| {
            // Query all eight advice columns at the current row.
            let queried = advice.map(|column| meta.query_advice(column, Rotation::cur()));
            let [a, b, c, d, e, f, g, h] = queried;
            let q_mul = meta.query_selector(q_mul);

            // Halo2 gates are polynomial expressions that must evaluate to 0.
            // When q_mul = 1, this enforces:
            // a * b * c * d * e * f * g = h.
            // This is a degree-7 custom gate, which is the new concept in this
            // exercise compared with the earlier add and mul gates.
            vec![q_mul * (a * b * c * d * e * f * g - h)]
        });

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
                    // Collect one logical row into column order so assignment
                    // can zip the homogeneous advice columns with their values.
                    let values = [row.a, row.b, row.c, row.d, row.e, row.f, row.g, row.h];

                    for (column, value) in config.advice.iter().zip(values) {
                        // Assign each value into the current row offset.
                        region.assign_advice(|| "value", *column, offset, || value)?;
                    }

                    // Enable the high-degree product gate on this row.
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
    // One private row that satisfies a * b * c * d * e * f * g = h passes.
    fn valid_witness() {
        let row = HighDegreeRow {
            a: Value::known(Fp::from(1)),
            b: Value::known(Fp::from(2)),
            c: Value::known(Fp::from(3)),
            d: Value::known(Fp::from(4)),
            e: Value::known(Fp::from(5)),
            f: Value::known(Fp::from(6)),
            g: Value::known(Fp::from(7)),
            h: Value::known(Fp::from(5040)),
        };
        let instance = vec![];

        let k = 4;
        let circuit = HighDegreeCircuit::<Fp, 1> { rows: [row] };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }
}
