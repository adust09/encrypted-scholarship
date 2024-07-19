pragma solidity ^0.8.0;

contract HybridConditionalWithdraw {
    address public owner;
    
    struct Deposit {
        uint256 amount;
        bool conditionMet;
        bool withdrawn;
    }

    mapping(address => Deposit[]) public deposits;

    event DepositCreated(address indexed depositor, uint256 amount, uint256 depositId);
    event ConditionMet(address indexed depositor, uint256 depositId);
    event WithdrawMade(address indexed depositor, address indexed recipient, uint256 amount, uint256 depositId);

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can call this function");
        _;
    }

    function deposit() public payable {
        require(msg.value > 0, "Deposit amount must be greater than 0");

        uint256 depositId = deposits[msg.sender].length;
        deposits[msg.sender].push(Deposit({
            amount: msg.value,
            conditionMet: false,
            withdrawn: false
        }));

        emit DepositCreated(msg.sender, msg.value, depositId);
    }

    function setConditionMet(address _depositor, uint256 _depositId) public onlyOwner {
        Deposit storage dep = deposits[_depositor][_depositId];
        require(!dep.conditionMet, "Condition has already been met");
        
        dep.conditionMet = true;
        emit ConditionMet(_depositor, _depositId);
    }

    function withdraw(address _depositor, uint256 _depositId, address payable _recipient) public {
        Deposit storage dep = deposits[_depositor][_depositId];
        require(dep.conditionMet, "Condition has not been met");
        require(!dep.withdrawn, "Deposit has already been withdrawn");
        
        dep.withdrawn = true;
        uint256 amount = dep.amount;
        dep.amount = 0; // Prevent re-entrancy

        _recipient.transfer(amount);

        emit WithdrawMade(_depositor, _recipient, amount, _depositId);
    }

    function getDeposit(address _depositor, uint256 _depositId) public view returns (uint256, bool, bool) {
        Deposit storage dep = deposits[_depositor][_depositId];
        return (dep.amount, dep.conditionMet, dep.withdrawn);
    }
}


// はい、コントラクトオーナーとブロックチェーンの外で計算を行うサーバーを同一のエンティティとすることは可能です。これは実際によく使われるアプローチで、オフチェーン計算とオンチェーン検証を組み合わせるハイブリッドシステムの一形態です。
// このアプローチの概要は以下のようになります：

// オフチェーン計算:

// サーバー（コントラクトオーナーが管理）が複雑な計算を実行します。


// 結果の報告:

// 計算結果がtrueの場合、サーバーがスマートコントラクトの関数を呼び出して結果を報告します。


// オンチェーン処理:

// スマートコントラクトが報告を受け取り、条件を満たしたとマークします。



// このアプローチを実装したスマートコントラクトの例を示します：

// このシステムの運用フローは以下のようになります：

// コントラクトのデプロイ:

// オーナー（サーバー運営者）がコントラクトをデプロイします。


// デポジット:

// Aliceがコントラクトに資金をデポジットします。


// オフチェーン計算:

// サーバーが必要な計算を実行します。


// 結果の報告:

// 計算結果がtrueの場合、サーバーがsetConditionMet関数を呼び出します。
// この呼び出しは、サーバーがコントラクトオーナーのプライベートキーを使用して行います。


// 引き出し:

// 条件が満たされた後、Bobがwithdraw関数を呼び出して資金を受け取ります。



// このアプローチの利点：

// 複雑な計算をオフチェーンで行うことができ、ガスコストを抑えられます。
// 単一のエンティティが計算と結果の報告を管理するため、システムが簡素化されます。



