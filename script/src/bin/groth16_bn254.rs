use sp1_sdk::{utils, HashableKey, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    let n = 10u32;
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    println!("vk: {:?}", vk.bytes32());

    let proof = client.prove(&pk, stdin).groth16().run().unwrap();

    let public_values = proof.public_values.as_slice();
    println!("public values: 0x{}", hex::encode(public_values));

    let solidity_proof = proof.bytes();
    println!("proof: 0x{}", hex::encode(solidity_proof));

    client.verify(&proof, &vk).expect("verification failed");

    proof
        .save("fibonacci-groth16.bin")
        .expect("saving proof failed");
}
