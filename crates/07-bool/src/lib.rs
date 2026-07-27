//! Placeholder for boolean constraint exercises.
//!
//! Goal: enforce that a value is either `0` or `1`.
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Expression, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug, Default)]
pub struct BoolCircuit<F: Field> {
    val: Value<F>,
}

#[derive(Clone, Debug)]
pub struct BoolConfig {
    val: Column<Advice>,
    q_bool: Selector,
}

impl<F: Field> Circuit<F> for BoolCircuit<F> {
    type Config = BoolConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let val = meta.advice_column();
        let q_bool = meta.selector();

        meta.create_gate("is-bool", |meta| {
            let val = meta.query_advice(val, Rotation::cur());
            let q_bool = meta.query_selector(q_bool);

            vec![q_bool * val.clone() * (val - Expression::Constant(F::ONE))]
        });

        BoolConfig { val, q_bool }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), plonk::Error> {
        layouter.assign_region(
            || "is-bool",
            |mut region| {
                region.assign_advice(|| "val", config.val, 0, || self.val)?;
                config.q_bool.enable(&mut region, 0)?;
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
    fn valid_witness_satisfies_constraint_one() {
        let val = Value::known(Fp::one());
        let instance = vec![];

        let k = 4;
        let circuit = BoolCircuit { val };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn valid_witness_satisfies_constraint_zero() {
        let val = Value::known(Fp::zero());
        let instance = vec![];

        let k = 4;
        let circuit = BoolCircuit { val };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn invalid_witness_is_err() {
        let val = Value::known(Fp::from(2));
        let instance = vec![];

        let k = 4;
        let circuit = BoolCircuit { val };
        let prover = MockProver::run(k, &circuit, instance).unwrap();
        assert!(prover.verify().is_err());
    }
}
