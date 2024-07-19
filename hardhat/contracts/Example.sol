// SPDX-License-Identifier: MIT 
pragma solidity ^0.8.0;

contract HashLockWithdraw {
    struct Deposit {
        uint256 amount;
        bytes32 hash;
        bool withdrawn;
    }

    mapping(address => Deposit[]) public deposits;

    event DepositCreated(address indexed depositor, uint256 amount, bytes32 hash, uint256 depositId);
    event WithdrawMade(address indexed depositor, address indexed recipient, uint256 amount, uint256 depositId);

    function deposit(bytes32 _hash) public payable {
        require(msg.value > 0, "Deposit amount must be greater than 0");

        uint256 depositId = deposits[msg.sender].length;
        deposits[msg.sender].push(Deposit({
            amount: msg.value,
            hash: _hash,
            withdrawn: false
        }));

        emit DepositCreated(msg.sender, msg.value, _hash, depositId);
    }

    function withdraw(address _depositor, uint256 _depositId, bytes32 _random) public {
        Deposit storage dep = deposits[_depositor][_depositId];
        require(!dep.withdrawn, "Deposit has already been withdrawn");
        require(keccak256(abi.encodePacked(_random)) == dep.hash, "Invalid random value");
        
        dep.withdrawn = true;
        uint256 amount = dep.amount;
        dep.amount = 0; // Prevent re-entrancy

        payable(msg.sender).transfer(amount);

        emit WithdrawMade(_depositor, msg.sender, amount, _depositId);
    }

    function getDeposit(address _depositor, uint256 _depositId) public view returns (uint256, bytes32, bool) {
        Deposit storage dep = deposits[_depositor][_depositId];
        return (dep.amount, dep.hash, dep.withdrawn);
    }
}
