// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract ScholarshipFund is ReentrancyGuard {
    using ECDSA for bytes32;

    address public owner;
    address public fheServerPubKey;
    uint256 public scholarshipAmount;
    
    mapping(address => bool) public hasApplied;
    mapping(address => bool) public isApproved;
    mapping(address => uint256) public donations;
    
    event Deposit(address indexed depositor, uint256 amount);
    event ScholarshipRequested(address indexed applicant);
    event ScholarshipApproved(address indexed applicant);
    event ScholarshipWithdrawn(address indexed recipient, uint256 amount);

    constructor(address _fheServerPubKey, uint256 _scholarshipAmount) {
        owner = msg.sender;
        fheServerPubKey = _fheServerPubKey;
        scholarshipAmount = _scholarshipAmount;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    function deposit() public payable {
        require(msg.value > 0, "Deposit amount must be greater than 0");
        donations[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    function requestScholarship(bytes memory signature) public nonReentrant {
        require(!hasApplied[msg.sender], "Already applied for scholarship");
        require(verifySignature(msg.sender, signature), "Invalid signature");
        
        hasApplied[msg.sender] = true;
        isApproved[msg.sender] = true;
        
        emit ScholarshipRequested(msg.sender);
        emit ScholarshipApproved(msg.sender);
    }

    function verifySignature(address applicant, bytes memory signature) internal view returns (bool) {
        bytes32 message = keccak256(abi.encodePacked(applicant));
        bytes32 signedMessage = message.toEthSignedMessageHash();
        address recoveredSigner = signedMessage.recover(signature);
        return recoveredSigner == fheServerPubKey;
    }

    function withdraw() public nonReentrant {
        require(isApproved[msg.sender], "Not approved for scholarship");
        require(address(this).balance >= scholarshipAmount, "Insufficient funds in contract");

        isApproved[msg.sender] = false;
        payable(msg.sender).transfer(scholarshipAmount);
        
        emit ScholarshipWithdrawn(msg.sender, scholarshipAmount);
    }

    function updateScholarshipAmount(uint256 newAmount) public onlyOwner {
        scholarshipAmount = newAmount;
    }

    function updateFHEServerPubKey(address newPubKey) public onlyOwner {
        fheServerPubKey = newPubKey;
    }

    function getContractBalance() public view returns (uint256) {
        return address(this).balance;
    }

    function getDonationAmount(address donor) public view returns (uint256) {
        return donations[donor];
    }

    function withdrawRemainingFunds() public onlyOwner {
        uint256 balance = address(this).balance;
        require(balance > 0, "No funds to withdraw");
        payable(owner).transfer(balance);
    }
}
