import { NextResponse } from "next/server";
import * as snarkjs from "snarkjs";
import fs from "fs";

export async function POST(req: Request) {
  try {
    const { bankBalance, gpa } = await req.json();

    // Specify the path to the compiled Circom circuit file
    // todo: Generate wasm and zkey files using trusted setup results
    // todo: Specify the path to the wasm file
    const wasmFile = "./co-circom/ScholarshipCheck_js/ScholarshipCheck.wasm";
    const zkeyFile = "./co-circom/ScholarshipCheck_0001.zkey";

    // Prepare input values
    const input = { bankBalance, gpa };

    // Generate proof
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(
      input,
      wasmFile,
      zkeyFile
    );

    // Verification
    const vKey = JSON.parse(
      fs.readFileSync("./path/to/your/verification_key.json", "utf8")
    );
    const verified = await snarkjs.groth16.verify(vKey, publicSignals, proof);

    return NextResponse.json({ proof, publicSignals, verified });
  } catch (error) {
    console.error("Error generating proof:", error);
    return NextResponse.json(
      { error: "Failed to generate proof" },
      { status: 500 }
    );
  }
}
