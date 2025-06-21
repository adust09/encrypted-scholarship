# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an encrypted scholarship system that enables privacy-preserving scholarship applications using zero-knowledge proofs and blockchain technology. The system allows students to apply for scholarships without revealing personal information to third parties, completing the entire process in less than a day compared to traditional 1-2 month processes.

## Architecture

The system consists of three main components:

1. **Frontend** (`/frontend`): Next.js application with Web3 integration for student applications
2. **Smart Contracts** (`/hardhat`): Ethereum smart contracts for fund management and verification  
3. **Zero-Knowledge Circuits** (`/co-circom`): Circom circuits for private eligibility verification

### Key Flow
1. Founders deposit funds into smart contract
2. Students submit bank balance/GPA via the frontend
3. ZK circuit verifies eligibility privately (balance below threshold, GPA requirements)
4. Smart contract verifies proof and transfers funds to eligible students

## Development Commands

### Frontend (Next.js)
```bash
cd frontend
npm run dev        # Start development server
npm run build      # Build for production
npm run start      # Start production server
npm run lint       # Run ESLint
```

### Smart Contracts (Hardhat)
```bash
cd hardhat
npx hardhat compile                                    # Compile contracts
npx hardhat test                                      # Run tests
npx hardhat node                                      # Start local network
npx hardhat ignition deploy ./ignition/modules/deploy.ts --network localhost  # Deploy contracts
```

### Zero-Knowledge Circuits (Circom)
The circuits are already compiled. Key files:
- `ScholarshipCheck.circom`: Main circuit for balance/GPA verification
- `ScholarshipCheck.wasm`: Compiled WASM for witness generation
- `pot12_final.ptau`: Powers of tau ceremony file for proof generation

## Tech Stack

- **Frontend**: Next.js 15, TypeScript, Tailwind CSS, Wagmi (Web3), Reown AppKit
- **Smart Contracts**: Solidity, Hardhat, OpenZeppelin
- **ZK Circuits**: Circom, circomlib for comparison operations
- **Blockchain**: Ethereum (currently local network only)

## Key Components

### Smart Contract (`hardhat/contracts/ScholarshipFund.sol`)
- Manages scholarship funds and donations
- Verifies signatures from FHE server for eligibility
- Handles fund distribution to approved students

### ZK Circuit (`co-circom/ScholarshipCheck.circom`)
- `CheckBalance`: Verifies if balance is below threshold using LessThan comparator
- `CheckHighestGPA`: Finds maximum GPA among applicants
- Uses 252-bit precision for better compatibility

### Frontend Form (`frontend/src/components/scholarship-form.tsx`)
- Collects bank balance and GPA from students
- Integrates with Web3 wallet connection
- Submits to `/api/evaluate-scholarship` endpoint

## Current State (MVP)

- Only bank balance evaluation is implemented
- TLS Notary for bank balance verification not yet implemented
- Smart contracts operate on local network only
- Some type conversions are hardcoded for convenience
- Frontend has Web3 integration but backend API needs implementation

## File Structure Notes

- `/doc`: Contains system documentation and architecture explanations
- `/memory-bank`: Contains ADRs (Architecture Decision Records) and project context
- `/todo.md`: Current development tasks