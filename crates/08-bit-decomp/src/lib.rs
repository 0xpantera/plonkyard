//! Homework 3 Problem 2: explicit bit witnesses.
//!
//! Goal:
//! Prove that a private value `v` is represented by four private witness bits.
//!
//! Constraints:
//! - each `bi` is boolean: `bi * (bi - 1) = 0`
//! - `v = b0 + 2*b1 + 4*b2 + 8*b3`
//!
//! Introduces:
//! - combining several polynomial constraints in one custom gate
//! - range checking through binary decomposition without lookups
//!
//! PLONKish concepts:
//! - a finite set encoded as the roots of a polynomial
//! - a linear combination tying auxiliary witnesses to a claimed value
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Expression, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug, Default)]
pub struct BitDecompCircuit<F: Field> {
    // The private value being decomposed.
    v: Value<F>,
    // Four private bits in little-endian order: bits[0] has weight 1 and
    // bits[3] has weight 8.
    bits: [Value<F>; 4],
}

#[derive(Clone, Debug)]
pub struct BitDecompConfig {
    // All five private values are assigned on one logical row.
    v: Column<Advice>,
    bits: [Column<Advice>; 4],
    // One selector is enough because every boolean check and the recomposition
    // constraint must be enabled on the same row.
    q: Selector,
}

impl<F: Field> Circuit<F> for BitDecompCircuit<F> {
    type Config = BitDecompConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare one advice column for v, four for its bits, and one
        // selector for the complete decomposition check.
        let v = meta.advice_column();
        let bits = std::array::from_fn(|_| meta.advice_column());
        let q = meta.selector();

        // 2. Define all five constraints for one logical decomposition row.
        meta.create_gate("bit-decomp", |meta| {
            let v = meta.query_advice(v, Rotation::cur());
            let queried = bits.map(|col| meta.query_advice(col, Rotation::cur()));
            let q = meta.query_selector(q);

            // Generate one independent boolean constraint for each claimed bit.
            // Cloning an Expression reuses its symbolic expression tree; it
            // does not clone concrete witness data.
            let boolean_constraint =
                |bit: Expression<F>| q.clone() * bit.clone() * (bit - Expression::Constant(F::ONE));

            let mut constraints = queried
                .iter()
                .cloned()
                .map(boolean_constraint)
                .collect::<Vec<_>>();

            let two = F::ONE.double();
            let four = two.double();
            let eight = four.double();

            // Interpret the queried bits as a little-endian binary number.
            let recomposed = queried[0].clone()
                + Expression::Constant(two) * queried[1].clone()
                + Expression::Constant(four) * queried[2].clone()
                + Expression::Constant(eight) * queried[3].clone();

            // The four boolean witnesses must reconstruct the claimed value.
            // Together, these constraints imply that v is in 0..16.
            constraints.push(q * (v - recomposed));
            constraints
        });

        BitDecompConfig { v, bits, q }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(
            || "bit-decomp",
            |mut region| {
                // Assign the claimed value and enable every gate expression on
                // this single logical row.
                region.assign_advice(|| "v", config.v, 0, || self.v)?;
                config.q.enable(&mut region, 0)?;

                // Pair each homogeneous bit column with its witness value. The
                // fallible iterator stops and returns the first assignment error.
                config
                    .bits
                    .iter()
                    .zip(self.bits.iter())
                    .enumerate()
                    .try_for_each(|(i, (column, bit))| {
                        region.assign_advice(|| format!("b{i}"), *column, 0, || *bit)?;
                        Ok::<(), plonk::Error>(())
                    })?;

                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::pasta::Fp;

    #[test]
    // Decimal 11 is binary 1011. The witness array is little-endian, so it is
    // stored as [1, 1, 0, 1].
    fn valid_witness_satisfies_constraints() {
        let v = Value::known(Fp::from(11));
        let bits = [
            Value::known(Fp::ONE),
            Value::known(Fp::ONE),
            Value::known(Fp::ZERO),
            Value::known(Fp::ONE),
        ];
        let instance = vec![];

        let k = 4;
        let circuit = BitDecompCircuit { v, bits };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    // The maximum four-bit value exercises all four binary weights.
    fn maximum_four_bit_value_satisfies_constraints() {
        let circuit = BitDecompCircuit {
            v: Value::known(Fp::from(15)),
            bits: [Value::known(Fp::ONE); 4],
        };

        let prover = MockProver::run(4, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    // These witnesses recompose to 3, but 3 is not a bit. This isolates the
    // boolean constraint from the recomposition constraint.
    fn non_boolean_bit_fails_constraints() {
        let circuit = BitDecompCircuit {
            v: Value::known(Fp::from(3)),
            bits: [
                Value::known(Fp::from(3)),
                Value::known(Fp::ZERO),
                Value::known(Fp::ZERO),
                Value::known(Fp::ZERO),
            ],
        };

        let prover = MockProver::run(4, &circuit, vec![]).unwrap();
        assert!(prover.verify().is_err());
    }

    #[test]
    // Every witness is a bit, but [1, 1, 0, 1] recomposes to 11 rather than
    // the claimed value 12. This isolates the recomposition constraint.
    fn incorrect_recomposition_fails_constraints() {
        let circuit = BitDecompCircuit {
            v: Value::known(Fp::from(12)),
            bits: [
                Value::known(Fp::ONE),
                Value::known(Fp::ONE),
                Value::known(Fp::ZERO),
                Value::known(Fp::ONE),
            ],
        };

        let prover = MockProver::run(4, &circuit, vec![]).unwrap();
        assert!(prover.verify().is_err());
    }
}
