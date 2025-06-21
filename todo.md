## Frontend

- [ ] Implement API integration for submitting scholarship applications (`frontend/src/api/route.ts`).
- [ ] Implement form validation in `frontend/src/components/scholarship-form.tsx`.
- [x] Limit GPA input to a maximum of 5 in form validation.
- [ ] Connect the form to the backend API.
- [x] Implement UI feedback (success/error messages) after form submission.
- [x] Use trusted setup results to generate wasm and zkey files.
- [ ] Specify the path to the wasm file: `co-circom/ScholarshipCheck_js/ScholarshipCheck.wasm`
- [ ] Specify the path to the zkey file: `co-circom/ScholarshipCheck_0001.zkey`
- [x] Integrate with Mematask using the `reown` library.

## Hardhat

- [ ] Implement the `ScholarshipFund.sol` contract logic.
- [ ] Write tests for the `ScholarshipFund.sol` contract.
- [ ] Enforce GPA limit of 5 in the smart contract.
- [ ] Deploy the contract to a test network.

## Co-circom

- [ ] Finalize the Circom circuit (`co-circom/ScholarshipCheck.circom`).
- [x] Generate the proving key and verification key.
- [ ] Integrate the Circom circuit with the smart contract.

## General

- [ ] Update documentation with implementation details.
