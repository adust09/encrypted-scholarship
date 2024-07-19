import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

export default buildModule("HashLockWithdrawModule", (m) => {
  const hashLockWithdraw = m.contract("HashLockWithdraw");

  return { hashLockWithdraw };
});
