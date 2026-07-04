//! Placeholder for Homework 2 Problem 1.
//!
//! Goal: enforce `a + b = c` with a custom gate over the first five rows.
//! Goal: privately assign five `(a, b, c)` rows and enforce `a + b = c`
//! on each row with a custom gate and selector.
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Selector},
    poly::Rotation,
};

#[derive(Clone, Copy)]
struct AddRow<F: Field> {
    // One logical row of private witness values.
    a: Value<F>,
    b: Value<F>,
    c: Value<F>,
}

pub struct AddRowsCircuit<F: Field> {
    // This exercise constrains exactly five rows. Each row is assigned into the
    // same advice columns at a different offset.
    rows: [AddRow<F>; 5],
}

#[derive(Clone, Debug)]
pub struct AddRowsConfig {
    // Advice columns hold witness values assigned by the prover.
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    // Selectors turn a gate on for specific rows.
    q_add: Selector,
}

impl<F: Field> Circuit<F> for AddRowsCircuit<F> {
    type Config = AddRowsConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        let add_row = AddRow {
            a: Value::unknown(),
            b: Value::unknown(),
            c: Value::unknown(),
        };

        Self { rows: [add_row; 5] }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare global circuit structure / column capabilities.
        // A simple selector is enough here because it only turns this one gate
        // on or off for each row.
        let q_add = meta.selector();
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();

        // 2. Define local algebraic constraints.
        meta.create_gate("add", |meta| {
            // Query the cells used by this gate at the current row.
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            let q_add = meta.query_selector(q_add);

            // Halo2 gates are polynomial expressions that must evaluate to 0.
            // When q_add = 1, this enforces: a + b = c.
            // When q_add = 0, the constraint is disabled for that row.
            vec![q_add * (a + b - c)]
        });

        AddRowsConfig { a, b, c, q_add }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(
            || "add",
            |mut region| {
                for (offset, row) in self.rows.iter().enumerate() {
                    // Assign each logical row into the same advice columns at
                    // a different offset. These closures return Value<F>,
                    // allowing witnesses to be known during proving and
                    // unknown during key generation.
                    region.assign_advice(|| "a", config.a, offset, || row.a)?;
                    region.assign_advice(|| "b", config.b, offset, || row.b)?;
                    region.assign_advice(|| "c", config.c, offset, || row.c)?;
                    // Enable the addition gate on this specific row.
                    config.q_add.enable(&mut region, offset)?;
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
    // Five private rows that each satisfy a + b = c pass.
    fn valid_witness_satisfies_constraints() {
        let rows = gen_valid_rows();
        let instance = vec![];

        let k = 4;
        let circuit = AddRowsCircuit { rows };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied()
    }

    #[test]
    // A single invalid row fails because q_add is enabled on every assigned row.
    fn invalid_row_fails_constraints() {
        let mut rows = gen_valid_rows();

        rows[3] = AddRow {
            a: Value::known(Fp::from(5)),
            b: Value::known(Fp::from(6)),
            c: Value::known(Fp::from(12)),
        };

        let circuit = AddRowsCircuit { rows };
        let prover = MockProver::run(4, &circuit, vec![]).unwrap();

        assert!(prover.verify().is_err());
    }

    fn gen_valid_rows() -> [AddRow<Fp>; 5] {
        let row0 = AddRow {
            a: Value::known(Fp::from(1)),
            b: Value::known(Fp::from(2)),
            c: Value::known(Fp::from(3)),
        };

        let row1 = AddRow {
            a: Value::known(Fp::from(3)),
            b: Value::known(Fp::from(4)),
            c: Value::known(Fp::from(7)),
        };

        let row2 = AddRow {
            a: Value::known(Fp::from(4)),
            b: Value::known(Fp::from(5)),
            c: Value::known(Fp::from(9)),
        };

        let row3 = AddRow {
            a: Value::known(Fp::from(5)),
            b: Value::known(Fp::from(6)),
            c: Value::known(Fp::from(11)),
        };

        let row4 = AddRow {
            a: Value::known(Fp::from(6)),
            b: Value::known(Fp::from(7)),
            c: Value::known(Fp::from(13)),
        };

        [row0, row1, row2, row3, row4]
    }
}
