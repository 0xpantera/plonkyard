//! Homework 3 Problem 1.
//!
//! Goal:
//! Prove that `N` private values form a Fibonacci sequence starting with
//! `1, 1`, using only one advice column.
//!
//! Constraints:
//! - `a[i + 2] = a[i + 1] + a[i]`
//! - `a[0] = a[1] = 1`
//!
//! Introduces:
//! - rotations for reading nearby rows from one column
//! - selectors that apply different constraints to different parts of a trace
//!
//! PLONKish concepts:
//! - expressing a recurrence as a local polynomial constraint
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Expression, Selector},
    poly::Rotation,
};

pub struct OneColFibCircuit<F: Field, const N: usize> {
    // The private Fibonacci trace. All N values are assigned vertically into
    // one advice column, with one value at each row offset.
    vals: [Value<F>; N],
}

impl<F: Field, const N: usize> Default for OneColFibCircuit<F, N> {
    fn default() -> Self {
        Self {
            // Preserve the N-row circuit shape while removing witness values
            // for key generation.
            vals: std::array::from_fn(|_| Value::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OneColFibConfig {
    // One advice column stores the complete sequence. Rotations let a gate read
    // three consecutive values without needing three separate columns.
    vals: Column<Advice>,
    // q_fib enables the recurrence only where two following rows exist.
    q_fib: Selector,
    // q_seed constrains the first two rows to the constant one.
    q_seed: Selector,
}

impl<F: Field, const N: usize> Circuit<F> for OneColFibCircuit<F, N> {
    type Config = OneColFibConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare the circuit's column and selectors.
        let vals = meta.advice_column();
        let q_fib = meta.selector();
        let q_seed = meta.selector();

        // 2. Define the recurrence as a local three-row constraint.
        meta.create_gate("recurrence", |meta| {
            // At row i, query a[i], a[i + 1], and a[i + 2] from the same
            // advice column using forward rotations.
            let a_cur = meta.query_advice(vals, Rotation::cur());
            let a_next = meta.query_advice(vals, Rotation::next());
            let a_next2 = meta.query_advice(vals, Rotation(2));
            let q_fib = meta.query_selector(q_fib);

            // When q_fib = 1, this expression must evaluate to zero:
            // a[i] + a[i + 1] = a[i + 2].
            vec![q_fib * (a_cur + a_next - a_next2)]
        });

        // 3. Define the initial-condition constraint separately because it
        // applies to different rows and reads only the current cell.
        meta.create_gate("seed_first_two_rows", |meta| {
            let a_cur = meta.query_advice(vals, Rotation::cur());
            let q_seed = meta.query_selector(q_seed);

            // Enabling q_seed on rows 0 and 1 enforces a[0] = a[1] = 1.
            vec![q_seed * (a_cur - Expression::Constant(F::ONE))]
        });

        OneColFibConfig {
            vals,
            q_fib,
            q_seed,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(
            || "one_col_fib",
            |mut region| {
                for (offset, row) in self.vals.iter().enumerate() {
                    // The seed gate applies only to the first two assigned rows.
                    if offset < 2 {
                        config.q_seed.enable(&mut region, offset)?;
                    }

                    // The forward-looking recurrence requires rows i + 1 and
                    // i + 2, so it must be disabled on the final two rows.
                    if offset + 2 < N {
                        config.q_fib.enable(&mut region, offset)?;
                    }

                    // Assign the private trace value into the one advice column
                    // at its corresponding row offset.
                    region.assign_advice(|| "value", config.vals, offset, || *row)?;
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
    // The required seeds and every recurrence step are valid.
    fn test_valid_witness() {
        let circuit = OneColFibCircuit::<Fp, 8> {
            vals: [
                Value::known(Fp::from(1)),
                Value::known(Fp::from(1)),
                Value::known(Fp::from(2)),
                Value::known(Fp::from(3)),
                Value::known(Fp::from(5)),
                Value::known(Fp::from(8)),
                Value::known(Fp::from(13)),
                Value::known(Fp::from(21)),
            ],
        };

        let instance = vec![];
        let k = 4;
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    // This is a valid Fibonacci recurrence, but the initial values are not the
    // required seeds 1 and 1. It must fail specifically because q_seed is
    // enabled on the first two rows.
    fn non_one_seeds_fail_constraints() {
        let circuit = OneColFibCircuit::<Fp, 8> {
            vals: [
                Value::known(Fp::from(2)),
                Value::known(Fp::from(3)),
                Value::known(Fp::from(5)),
                Value::known(Fp::from(8)),
                Value::known(Fp::from(13)),
                Value::known(Fp::from(21)),
                Value::known(Fp::from(34)),
                Value::known(Fp::from(55)),
            ],
        };

        let prover = MockProver::run(4, &circuit, vec![]).unwrap();
        assert!(prover.verify().is_err());
    }
}
