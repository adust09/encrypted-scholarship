import { NextResponse } from "next/server";

export async function POST(req: Request) {
  try {
    const { bankBalance, gpa, walletAddress } = await req.json();

    // Validate input
    if (typeof bankBalance !== 'number' || typeof gpa !== 'number' || !walletAddress) {
      return NextResponse.json(
        { error: "Invalid input data" },
        { status: 400 }
      );
    }

    // For MVP: Simple eligibility check
    // Criteria: Bank balance must be less than $50,000 and GPA >= 3.0
    const BALANCE_THRESHOLD = 50000;
    const GPA_THRESHOLD = 3.0;

    const isEligible = bankBalance < BALANCE_THRESHOLD && gpa >= GPA_THRESHOLD;

    return NextResponse.json({
      approved: isEligible,
      walletAddress,
      bankBalance,
      gpa,
      criteria: {
        balanceThreshold: BALANCE_THRESHOLD,
        gpaThreshold: GPA_THRESHOLD,
        balanceCheck: bankBalance < BALANCE_THRESHOLD,
        gpaCheck: gpa >= GPA_THRESHOLD
      }
    });

  } catch (error) {
    console.error("Error evaluating scholarship:", error);
    return NextResponse.json(
      { error: "Failed to evaluate scholarship application" },
      { status: 500 }
    );
  }
}