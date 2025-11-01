# Repository Guidelines

## Project Structure & Module Organization
- `frontend/` hosts the Next.js 15 app (App Router in `frontend/src/app`), shared UI in `frontend/src/components`, and helpers in `frontend/src/lib`. Static assets sit in `frontend/public`.
- `hardhat/` contains the Solidity sources (`contracts/`), deployment flows (`ignition/`), and the Hardhat config.
- `co-circom/` stores the Circom circuit plus proving and verification assets consumed by the app and contracts.
- `doc/` collects architecture and concept notes. Update it whenever you change flows so docs stay in sync.
- Review `CLAUDE.md` with this guide before planning multi-directory work.

## Build, Test, and Development Commands
- Frontend: `cd frontend && npm install && npm run dev` for local work, `npm run build` for production checks, and `npm run lint` before every PR.
- Contracts: `cd hardhat && npm install`, then `npx hardhat compile`, `npx hardhat test`, and when needed `npx hardhat node` followed by `npx hardhat ignition deploy ./ignition/modules/Lock.ts --network localhost`.
- Circuits: `cd co-circom && snarkjs groth16 verify verification_key.json public.json proof.json` whenever circuits or keys change.

## Coding Style & Naming Conventions
- TypeScript/React: keep strict types, use function components, and mark client modules with `"use client"` only when required. Favor camelCase for files/exports, kebab-case for route folders, and colocate reusable UI inside `frontend/src/components`.
- Tailwind CSS drives styling; prefer utility classes and limit global overrides.
- Solidity: include SPDX headers, `pragma solidity ^0.8.x`, four-space indentation, camelCase for functions/state, and emit events for every state change.

## Testing Guidelines
- Grow Hardhat coverage under `hardhat/test`; mirror contract methods (e.g., `ScholarshipFund.withdraw` → `withdraw.spec.ts`) and assert both events and state.
- Frontend currently ships without automated tests; at minimum run `npm run lint`, and add Jest/Testing Library cases under `frontend/src/__tests__` when adding business logic.
- When circuits evolve, regenerate artifacts with `snarkjs groth16 prove ...` and commit new `.zkey` or `verification_key.json` only after a passing `snarkjs groth16 verify`.

## Commit & Pull Request Guidelines
- Match the existing sentence-style commit subjects (e.g., “Enhance scholarship evaluation API and result page layout”) and cover the main surfaces you touched.
- PRs should summarize the change, link issues, attach screenshots or terminal output for UI or CLI work, and state the checks you ran.
- Keep scope tight—split circuit, contract, and frontend edits unless a feature genuinely spans them.
