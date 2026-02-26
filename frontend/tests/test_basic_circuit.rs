use frontend::layouter::builder::{CircuitBuilder, LayerVisibility};
use remainder::{
    mle::evals::MultilinearExtension,
    prover::helpers::{
        prove_circuit_with_runtime_optimized_config, test_circuit_with_runtime_optimized_config,
        verify_circuit_with_proof_config,
    },
};
use shared_types::transcript::poseidon_sponge::PoseidonSponge;
use shared_types::Fr;

fn build_test_circuit() -> frontend::layouter::builder::Circuit<Fr> {
    let mut builder = CircuitBuilder::<Fr>::new();

    let input_layer = builder.add_input_layer("Input Layer", LayerVisibility::Public);

    let input_shred_1 = builder.add_input_shred("Input 1", 2, &input_layer);
    let input_shred_2 = builder.add_input_shred("Input 2", 2, &input_layer);

    let sector_1 = builder.add_sector(&input_shred_1 + &input_shred_2);
    let sector_2 = builder.add_sector(&input_shred_1 * &input_shred_2);
    let out_sector = builder.add_sector(sector_1 * sector_2);

    let expected_output_shred = builder.add_input_shred("Expected Output", 2, &input_layer);
    let final_sector = builder.add_sector(out_sector - expected_output_shred);
    builder.set_output(&final_sector);

    builder.build_with_layer_combination().unwrap()
}

#[test]
fn test_basic_circuit() {
    let mut circuit = build_test_circuit();

    circuit.set_input("Input 1", vec![1, 1, 1, 1].into());
    circuit.set_input("Input 2", vec![16, 16, 16, 16].into());
    circuit.set_input(
        "Expected Output",
        vec![16 * 17, 16 * 17, 16 * 17, 16 * 17].into(),
    );

    let provable_circuit = circuit.gen_provable_circuit().unwrap();

    test_circuit_with_runtime_optimized_config(&provable_circuit);
}

#[test]
fn test_independent_prove_verify_builds() {
    let input_1: MultilinearExtension<Fr> = vec![1, 1, 1, 1].into();
    let input_2: MultilinearExtension<Fr> = vec![16, 16, 16, 16].into();
    let expected: MultilinearExtension<Fr> = vec![16 * 17, 16 * 17, 16 * 17, 16 * 17].into();

    let mut prover_circuit = build_test_circuit();
    prover_circuit.set_input("Input 1", input_1.clone());
    prover_circuit.set_input("Input 2", input_2.clone());
    prover_circuit.set_input("Expected Output", expected.clone());
    let provable = prover_circuit.gen_provable_circuit().unwrap();
    let (proof_config, transcript) =
        prove_circuit_with_runtime_optimized_config::<Fr, PoseidonSponge<Fr>>(&provable);

    let mut verifier_circuit = build_test_circuit();
    verifier_circuit.set_input("Input 1", input_1);
    verifier_circuit.set_input("Input 2", input_2);
    verifier_circuit.set_input("Expected Output", expected);
    let verifiable = verifier_circuit.gen_verifiable_circuit().unwrap();
    verify_circuit_with_proof_config(&verifiable, &proof_config, transcript);
}
