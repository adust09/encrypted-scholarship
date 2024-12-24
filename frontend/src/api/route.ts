import { NextResponse } from "next/server";
import * as snarkjs from "snarkjs";
import fs from "fs";

export async function POST(req: Request) {
  try {
    const { bankBalance, gpa } = await req.json();

    // CircomのコンパイルされたCircuitファイルのパスを指定
    const wasmFile = "./path/to/your/circuit.wasm";
    const zkeyFile = "./path/to/your/circuit_final.zkey";

    // 入力値の準備
    const input = { bankBalance, gpa };

    // Proofの生成
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(
      input,
      wasmFile,
      zkeyFile
    );

    // 検証
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
