//! Placeholder for Homework 3 Problem 2 bit decomposition.
//!
//! Goal: decompose `v` into four witness bits.
use ff::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Expression, Selector},
    poly::Rotation,
};

#[derive(Clone, Debug, Default)]
pub struct BitDecompCircuit<F: Field> {
    v: Value<F>,
    bits: [Value<F>; 4],
}

#[derive(Clone, Debug)]
pub struct BitDecompConfig {
    v: Column<Advice>,
    bits: [Column<Advice>; 4],
    q: Selector,
}

impl<F: Field> Circuit<F> for BitDecompCircuit<F> {
    type Config = BitDecompConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let v = meta.advice_column();
        let bits = std::array::from_fn(|_| meta.advice_column());
        let q = meta.selector();

        meta.create_gate("bit-decomp", |meta| {
            let v = meta.query_advice(v, Rotation::cur());
            let queried = bits.map(|col| meta.query_advice(col, Rotation::cur()));
            let q = meta.query_selector(q);

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

            let recomposed = queried[0].clone()
                + Expression::Constant(two) * queried[1].clone()
                + Expression::Constant(four) * queried[2].clone()
                + Expression::Constant(eight) * queried[3].clone();

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
                region.assign_advice(|| "v", config.v, 0, || self.v)?;
                config.q.enable(&mut region, 0)?;

                for (i, (col, bit)) in config.bits.iter().zip(self.bits.iter()).enumerate() {
                    region.assign_advice(|| format!("b{i}"), *col, 0, || *bit)?;
                }

                Ok(())
            },
        )
    }
}
