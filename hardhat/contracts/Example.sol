// SPDX-License-Identifier: MIT 
pragma solidity ^0.8.0;

contract HashLockWithdraw {
    struct Deposit {
        address depositor;
        uint256 amount;
        bytes32 hash;
        bool withdrawn;
    }

    mapping(address => Deposit[]) public deposits;

    event DepositCreated(address indexed depositor, uint256 amount, uint256 depositId);
    event HashSet(address indexed depositor, uint256 depositId, bytes32 hash);
    event WithdrawMade(address indexed depositor, address indexed recipient, uint256 amount, uint256 depositId);

    function deposit() public payable {
        require(msg.value > 0, "Deposit amount must be greater than 0");

        uint256 depositId = deposits[msg.sender].length;
        deposits[msg.sender].push(Deposit({
            depositor: msg.sender,
            amount: msg.value,
            hash: bytes32(0),
            withdrawn: false
        }));

        emit DepositCreated(msg.sender, msg.value, depositId);
    }

    function setHash(address depositor, uint256 depositId, bytes32 _hash) public {
        require(deposits[depositor][depositId].hash == bytes32(0), "Hash already set");
        deposits[depositor][depositId].hash = _hash;
        emit HashSet(depositor, depositId, _hash);
    }

    function withdraw(address depositor, uint256 depositId, bytes32 _random) public {
        Deposit storage dep = deposits[depositor][depositId];
        require(!dep.withdrawn, "Deposit has already been withdrawn");
        require(dep.hash != bytes32(0), "Hash not set");
        require(keccak256(abi.encodePacked(_random)) == dep.hash, "Invalid random value");
        
        dep.withdrawn = true;
        uint256 amount = dep.amount;
        dep.amount = 0; // Prevent re-entrancy

        payable(msg.sender).transfer(amount);

        emit WithdrawMade(depositor, msg.sender, amount, depositId);
    }
}
