use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Instance, Selector},
    poly::Rotation,
};

// Public: out
// Private: a, b
// Constraint: a * b = out
// Example witness:
// a = 7
// b = 8
// out = 56
pub struct MulCircuit<F: Field> {
    a: Value<F>,
    b: Value<F>,
    out: Value<F>,
}

#[derive(Clone, Debug)]
pub struct MulConfig {
    // Advice columns hold witness values assigned by the prover.
    a: Column<Advice>,
    b: Column<Advice>,
    // `out` is also assigned as advice so the multiplication gate can read it.
    out: Column<Advice>,
    // Instance columns hold public inputs supplied from outside the circuit.
    instance: Column<Instance>,
    // Selectors turn a gate on for specific rows.
    q_enable: Selector,
}

impl<F: Field> Circuit<F> for MulCircuit<F> {
    type Config = MulConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            a: Value::unknown(),
            b: Value::unknown(),
            out: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // 1. Declare global circuit structure / column capabilities.
        let q_enable = meta.complex_selector();
        let a = meta.advice_column();
        let b = meta.advice_column();
        let out = meta.advice_column();
        let instance = meta.instance_column();

        // Equality is required for cells that will be constrained equal through
        // Halo2's permutation argument. Here we expose `out` by constraining an
        // advice cell equal to a public instance cell.
        meta.enable_equality(out);
        meta.enable_equality(instance);

        // 2. Define local algebraic constraints.
        meta.create_gate("mul", |meta| {
            // Query the cells used by this gate at the current row.
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let out = meta.query_advice(out, Rotation::cur());
            let q_enable = meta.query_selector(q_enable);

            // Halo2 gates are polynomial expressions that must evaluate to 0.
            // When q_enable = 1, this enforces: a * b = out.
            // When q_enable = 0, the constraint is disabled for that row.
            vec![q_enable * (a * b - out)]
        });

        MulConfig {
            a,
            b,
            out,
            instance,
            q_enable,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        let out_cell = layouter.assign_region(
            || "mul",
            |mut region| {
                // Assign witness values into row 0 of the advice columns.
                // These closures return Value<F>, allowing witnesses to be
                // known during proving and unknown during key generation.
                region.assign_advice(|| "assign_advice", config.a, 0, || self.a)?;
                region.assign_advice(|| "assign_advice", config.b, 0, || self.b)?;
                let out_cell =
                    region.assign_advice(|| "assign advice", config.out, 0, || self.out)?;
                // Enable the multiplication gate on row 0.
                config.q_enable.enable(&mut region, 0)?;
                Ok(out_cell)
            },
        )?;

        // Public inputs are not assigned inside regions. Instead, the prover
        // supplies them separately, and the circuit constrains this advice cell
        // to equal instance column 0, row 0.
        layouter.constrain_instance(out_cell.cell(), config.instance, 0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::pasta::Fp;

    #[test]
    fn valid_witness_satisfies_constraints() {
        let a = Value::known(Fp::from(7));
        let b = Value::known(Fp::from(8));
        let out = Fp::from(56);
        let circuit_out = Value::known(out);
        let public_inputs = vec![vec![out]];

        let k = 4;

        let circuit = MulCircuit {
            a,
            b,
            out: circuit_out,
        };
        let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn wrong_output_fails_constraints() {
        let a = Value::known(Fp::from(7));
        let b = Value::known(Fp::from(8));
        let out = Fp::from(55);
        let circuit_out = Value::known(out);
        let public_inputs = vec![vec![out]];

        let k = 4;

        let circuit = MulCircuit {
            a,
            b,
            out: circuit_out,
        };
        let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
        assert!(prover.verify().is_err());
    }
}
