//! Placeholder for Homework 2 Problem 2.
//!
//! Goal: alternate addition and multiplication gates over eight rows.
//! Even rows enforce `a + b = c`; odd rows enforce `a * b = c`.
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Selector},
    poly::Rotation,
};

#[derive(Clone, Copy)]
struct AddMulRow<F: Field> {
    // One logical row of private witness values.
    a: Value<F>,
    b: Value<F>,
    c: Value<F>,
}

pub struct AddMulRowsCircuit<F: Field> {
    // This exercise constrains exactly eight rows. The same advice columns are
    // reused on every row, but different selectors choose the active operation.
    rows: [AddMulRow<F>; 8],
}

#[derive(Clone, Debug)]
pub struct AddMulRowsConfig {
    // Advice columns hold witness values assigned by the prover.
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    // Two selectors let the circuit choose which gate is active on each row.
    q_add: Selector,
    q_mul: Selector,
}

impl<F: Field> Circuit<F> for AddMulRowsCircuit<F> {
    type Config = AddMulRowsConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        let add_mul_row = AddMulRow {
            a: Value::unknown(),
            b: Value::unknown(),
            c: Value::unknown(),
        };

        Self {
            rows: [add_mul_row; 8],
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare global circuit structure / column capabilities.
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        // Simple selectors are enough here because each one only turns one
        // constraint on or off for a row.
        let q_add = meta.selector();
        let q_mul = meta.selector();

        // 2. Define local algebraic constraints.
        // This single gate returns two independent expressions. It is still
        // conceptually two gated constraints: one for addition, one for
        // multiplication.
        meta.create_gate("add_or_mul", |meta| {
            // Query the cells used by both constraints at the current row.
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            let q_add = meta.query_selector(q_add);
            let q_mul = meta.query_selector(q_mul);

            // Halo2 gates are polynomial expressions that must evaluate to 0.
            // When q_add = 1, this enforces: a + b = c.
            // When q_mul = 1, this enforces: a * b = c.
            // Returning separate expressions avoids letting the add and mul
            // constraints cancel each other out.
            vec![
                q_add * (a.clone() + b.clone() - c.clone()),
                q_mul * (a * b - c),
            ]
        });

        AddMulRowsConfig {
            a,
            b,
            c,
            q_add,
            q_mul,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(
            || "add_or_mul",
            |mut region| {
                for (offset, row) in self.rows.iter().enumerate() {
                    // Alternate row modes: even rows add, odd rows multiply.
                    if offset % 2 == 0 {
                        config.q_add.enable(&mut region, offset)?;
                    } else {
                        config.q_mul.enable(&mut region, offset)?;
                    }

                    // Assign each logical row into the same advice columns at
                    // a different offset. The active selector decides which
                    // equation constrains these three cells.
                    region.assign_advice(|| "a", config.a, offset, || row.a)?;
                    region.assign_advice(|| "b", config.b, offset, || row.b)?;
                    region.assign_advice(|| "c", config.c, offset, || row.c)?;
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
    // Eight private rows pass when even rows satisfy addition and odd rows
    // satisfy multiplication.
    fn valid_witness_satisfies_constraints() {
        let rows = gen_valid_rows();
        let instance = vec![];

        let k = 4;
        let circuit = AddMulRowsCircuit { rows };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    // An invalid even row fails because q_add is enabled on that row.
    fn add_row_fails_addition() {
        let mut rows = gen_valid_rows();
        let instance = vec![];

        // Should be 9
        rows[2].c = Value::known(Fp::from(10));

        let k = 4;
        let circuit = AddMulRowsCircuit { rows };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        assert!(prover.verify().is_err());
    }

    #[test]
    // An invalid odd row fails because q_mul is enabled on that row.
    fn mul_row_fails_multiplication() {
        let mut rows = gen_valid_rows();
        let instance = vec![];

        // Should be 30
        rows[3].c = Value::known(Fp::from(31));

        let k = 4;
        let circuit = AddMulRowsCircuit { rows };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        assert!(prover.verify().is_err());
    }

    #[test]
    // A row that satisfies the wrong operation still fails. Row 1 is a
    // multiplication row, so setting c = a + b is not accepted.
    fn wrong_op_fails() {
        let mut rows = gen_valid_rows();
        let instance = vec![];

        rows[1] = AddMulRow {
            a: Value::known(Fp::from(3)),
            b: Value::known(Fp::from(4)),
            c: Value::known(Fp::from(7)),
        };

        let k = 4;
        let circuit = AddMulRowsCircuit { rows };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        assert!(prover.verify().is_err());
    }

    fn gen_valid_rows() -> [AddMulRow<Fp>; 8] {
        let row0 = AddMulRow {
            a: Value::known(Fp::from(1)),
            b: Value::known(Fp::from(2)),
            c: Value::known(Fp::from(3)),
        };

        let row1 = AddMulRow {
            a: Value::known(Fp::from(3)),
            b: Value::known(Fp::from(4)),
            c: Value::known(Fp::from(12)),
        };

        let row2 = AddMulRow {
            a: Value::known(Fp::from(4)),
            b: Value::known(Fp::from(5)),
            c: Value::known(Fp::from(9)),
        };

        let row3 = AddMulRow {
            a: Value::known(Fp::from(5)),
            b: Value::known(Fp::from(6)),
            c: Value::known(Fp::from(30)),
        };

        let row4 = AddMulRow {
            a: Value::known(Fp::from(6)),
            b: Value::known(Fp::from(7)),
            c: Value::known(Fp::from(13)),
        };

        let row5 = AddMulRow {
            a: Value::known(Fp::from(7)),
            b: Value::known(Fp::from(8)),
            c: Value::known(Fp::from(56)),
        };

        let row6 = AddMulRow {
            a: Value::known(Fp::from(8)),
            b: Value::known(Fp::from(9)),
            c: Value::known(Fp::from(17)),
        };

        let row7 = AddMulRow {
            a: Value::known(Fp::from(9)),
            b: Value::known(Fp::from(10)),
            c: Value::known(Fp::from(90)),
        };

        [row0, row1, row2, row3, row4, row5, row6, row7]
    }
}
