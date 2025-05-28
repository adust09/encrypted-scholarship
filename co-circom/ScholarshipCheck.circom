pragma circom 2.2.1;

include "../node_modules/circomlib/circuits/comparators.circom";
include "../node_modules/circomlib/circuits/switcher.circom";

template Max(n) {
    signal input in[n];
    signal output out;

    component nbs[n];        // store bit-length checkers
    component gts[n];        // store comparators
    component switchers[n+1];  // switcher for comparing maxs

    signal maxs[n+1];

    for(var i = 0; i < n; i++) {
        nbs[i] = Num2Bits(252);
        nbs[i].in <== in[i];
    }

    maxs[0] <== in[0];
    for(var i = 0; i < n; i++) {
        gts[i] = GreaterThan(252); // changed to 252 (maximum) for better compatibility
        switchers[i+1] = Switcher();

        gts[i].in[1] <== maxs[i];
        gts[i].in[0] <== in[i];

        switchers[i+1].sel <== gts[i].out;
        switchers[i+1].L <== maxs[i];
        switchers[i+1].R <== in[i];

        maxs[i+1] <== switchers[i+1].outL;
    }

    out <== maxs[n];
}

template CheckBalance(n) {
    signal input balance[n];
    signal input threshold;
    signal output isBalanceLow[n];

    component lts[n];

    for (var i = 0; i < n; i++) {
        lts[i] = LessThan(252);
        lts[i].in[0] <== balance[i];
        lts[i].in[1] <== threshold;
        isBalanceLow[i] <== lts[i].out;
    }
}

template CheckHighestGPA(n) {
    signal input gpa[n];
    signal output maxGPA;

    component max = Max(n);
    max.in <== gpa;
    maxGPA <== max.out;
}

template FindMaxGPA(n) {
    signal input gpa[n];
    signal output maxGPA;

    component max = Max(n);
    max.in <== gpa;
    maxGPA <== max.out;
}

template ScholarshipCheck() {
    signal input balance[4];
    signal input gpa[4];
    signal input threshold;

    signal output eligibleStudentIndex;

    component checkBalance = CheckBalance(4);
    checkBalance.balance <== balance;
    checkBalance.threshold <== threshold;

    component checkGPA = CheckHighestGPA(4);
    checkGPA.gpa <== gpa;
}

component main = ScholarshipCheck();
