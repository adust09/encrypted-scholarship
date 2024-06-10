# 1. Introduction

This is a scholarship support system between anonymous individuals.

Students provide information, and their applications are evaluated through secure computation using private and verifiable FHE computation. Scholarships are awarded to students who meet the criteria, and the funds are transferred. Building this system on Ethereum enables the possibility of scholarships crossing borders.

# 2. Background - Scholarship for Higher Education

To expand access to higher education, various scholarships are provided by governments, private organizations, and educational institutions, which have diverse evaluation criteria such as academic performance, specific talents, and family economic situations.

However, the following challenges(2.1~2.3) are faced in the system:

## 2.1 **Repayment Issues**

Tuition fees at many universities are increasing, becoming a significant burden for students and their families, leading to an increased demand for scholarships. Students who utilize loan-based scholarships are required to repay them after graduation. However, due to employment conditions or economic situations, some students may be unable to repay and may file for bankruptcy.

## 2.2 **Ensuring Equal Access**

Outstanding scholarships are limited, and scholarships for students with excellent academic records or specific talents are highly competitive. It is important to provide support to students from economically disadvantaged backgrounds or with specific circumstances. However, it is difficult for them to succeed in intense competition. Ensuring equal access to scholarships for all students remains a challenge.

## 2.3 **Sudden Environmental Changes**

The outbreak of the novel coronavirus has impacted students' economic situations and increased the demand for scholarships.

Additionally, there are new expenses associated with the transition to online education. Public support systems are determined by socioeconomic factors and educational policies, making it difficult to improve smoothly in the face of rapidly changing student environments.

## 2.4 **Expectation**
The evaluation is based on the student's "economic situation" and "creditworthiness," but it can be imagined that these criteria will increase as the demand expands while resources remain limited.

This trend may lead to the inability to provide educational opportunities to low-income households, creating a negative spiral.


## 2.5 Solution


What is important there is whether students who have difficulty in obtaining social credit can 'prove their creditworthiness' and attract a large number of supporters.

Ethereum makes it possible.

Several cryptographic theories would be used to guarantee the credit of students, and smart contracts would complete the provision and receipt of funds.

In particular, it can be combined with zkFHE as a means of protecting the privacy of students for secure screening.

This enables cross-border study support in a world where everyone is anonymous.

Proposing more scholarship programs can be one solution to increase students' options.
Individual support can function as a flexible system for this purpose.Just like in "Daddy-Long-Legs."

# 3. General system architecture
Students provide information about their financial situation and social credits, and the endorser evaluate them (this evaluation will be done privately) to decide whether the students are qualified for the financial support. 


The process would flow as follows:

1. Supporters present their own evaluation criteria and the amount of support they offer, and deposit the corresponding assets into a smart contract.

2. Students (and other stakeholders) who want to apply for the scholarship provide the necessary numeric values to the evaluation criteria. 

    Note: It is not necessarily the case that the students alone provide the information. For example, the school that the students attend may provide information about their academic performance, and non-profit organizations may provide information about their contributions in extracurricular volunteer activities.
 
3. The Fund server evaluates whether the criteria are met through homomorphic computation.

4. If the criteria are met, the deposited support funds in the smart contract are transferred to the student's address.

Asume that there is no need to reapay this scholarship. 

# 4. Example

## 4.1 Launching the Alice STEM Scholarship Initiative

Alice is a wealthy individual who believes in promoting STEM education.

With the intention of promoting the advancement of computer science (CS), she has announced the following scholarships:

> **Alice STEM scholarship**
> 
> - Provide 50K USDC
> - 18 years old or older
> - Household annual income of $60K or less
> 
> Meet one of the following conditions:
> 
> - A: Enrolled in a CS educational institution
> - B: Alice has achieved criteria set by herself on Github
>     - How many commits have been made
>     - How much contribution has been made to other projects
>     - His recognition as an engineer

Alice deposits 50K USDC into the scholarship contract.

## 4.2 Bob applies to Alice STEM Scholarship

Bob is a high school student who develops software on his own and contributes to open source projects afterschool.

He wishes to attend a prestigious university known for computer science in the future. 

However, his single mother, who works at a supermarket, supports the family financially and they are unable to afford even the tuition fees.

Therefore, he has come up with a plan to secure the tuition fees using Alice's scholarship program B.
(In reality, Bob will likely combine other scholarship programs to cover tuition and living expenses.)


Each stakeholder will submit information about Bob to the fund server.
| Bob               |      |
| ----------------- | ---- |
| recipient_address | 0x…. |
| age               | 18   |

| Supermarket  |      |
| ------------ | ---- |
| anual_salary | $50K |


| Github        |     |
| ------------- | --- |
| commits       | 300 |
| contributions | 500 |
| stars         | 50  |
| followers     | 40  |

## 4.3 Screening with zkFHE

Using the provided information, Alice executes `evaluate_github_criteria` to determine if Bob is eligible for the scholarship. 

```rust
struct GithubInfo {
    commits: u32,
    contributions: u32,
    stars: u32,
    followers: u32,
}

fn evaluate_github_criteria(github_info: &GithubInfo) -> bool {
    github_info.commits >= 100 && 
		github_info.contributions >= 50 && 
		(github_info.stars >= 20 || github_info.followers >= 40)
}

fn evaluate_scholarship(age: u32, household_income: u32, enrolled_in_cs: bool, github_info: GithubInfo) -> u32 {
    if age < 18 || household_income > 60000 {
        return 0;
    }

    if enrolled_in_cs || evaluate_github_criteria(&github_info) {
        return 1;
    }

    0
}

fn main() {
    let bob = evaluate_scholarship(18, 50000, true, GithubInfo { commits: 0, contributions: 0, endorsements: 0, influence: 0 });
    println!("Result of bob", bob); 
```
~~This function itself is the part to be implemented in zkFHE~~. 
This process is executed using zkFHE.
Bob wants to keep his salary and age secret.

## 4.4 Acceptance

If true, the fund server will transfer Bob 50K USDC through the scholarship contract.


# 5. Issue remained

## 5.1 Requirements
Additionally, in order to design a sound system, the following requirements need to be considered:

- A mechanism is needed to prevent students from obtaining benefits through fraudulent means.
- ~~How to ensure the accuracy of information obtained from stakeholders.~~
- A mechanism is needed to verify that the funds are used properly for tuition and enrollment fees.
- Is it possible to operate a smart contract based on zkFHE results?

## 5.2 Improvements
Although not proposed this time, the following improvements seem possible:

- Alice is only the initiator and one of the investors, and it is possible that people who agree with Alice's idea will co-invest
- It may be possible to extende this system as personal donations or crowdfunding.
- It would be good to prepare a fund like the Scholarship Pool and have volunteers contribute to it.
- The financial aid (needs to be repaid) system functions similar to a lending protocol. This could create another risk, but it is experimental and interesting.
- It is also possible to have regular payments made per semester (tracking grades would be necessary?).

# 6. Prototype(wip)
## 6.1 What to do
1. Notarize your bank balance with TSLN
2. Calculate student scores based on bank balances
3. Disclose scores to students
![image](https://hackmd.io/_uploads/r11dZ_ioT.png)

### 6.1.1 TLSN
There are two ways

1. [Rust](https://docs.tlsnotary.org/quick_start/rust.html) : Authorization tokens etc. need to be registered in `.env` in advance. 
It is a bit hard to find this, but any bank will be able to handle it.
2. [TLSNotary Browser Extension](https://docs.tlsnotary.org/quick_start/browser_extension.html) : This would likely reduce the cost of prototype development, but it is limited to banks that open their APIs to the public.


The TLSN team seem to be looking for a use case, so I'll discuss it with them.
https://discord.com/channels/974410303255232542/1167483766122500196

### 6.1.2 Scoreing algorithm
For starters, it might be better to just set a random threshold and determine if the balance is above that.

## 6.2 What not to do
- Funders deposit(distribute funds to students)
- Obtaining parameters other than bank balance
