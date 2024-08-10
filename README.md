
# ICP Token Wallet

## Overview

The ICP Token Wallet is a Rust-based wallet built for the Internet Computer Protocol (ICP) blockchain. It supports basic functionalities such as sending and receiving IRCRC2 tokens and displaying the current token balance. This project showcases proficiency in Rust and blockchain principles by implementing a simple, secure token wallet.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Setup](#setup)
- [Usage](#usage)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)


## Features

- **Send Tokens:** Transfer tokens to other addresses.
- **Receive Tokens:** Receive tokens and update the wallet balance.
- **Balance Display:** Show the current token balance.
- **Basic Security:** Implement basic wallet security measures, such as signature verification.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [DFX SDK](https://internetcomputer.org/install.sh) for ICP development

### Clone the Repository

<!-- 
git clone https://github.com/lavishn01/icp-token-wallet.git
cd icp-token-wallet -->


### Install Rust Dependencies


<!-- cargo build -->


### Install DFX SDK

If you don't have the DFX SDK installed, install it using the following command:


<!-- sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)" -->


## Setup

### 1. Start the Local ICP Test Network

Before deploying the smart contract, ensure that the ICP test network is running:


<!-- dfx start --background -->


### 2. Deploy the Canister

Deploy the smart contract to the local test network:


<!-- dfx deploy -->


This will deploy the canister and output the canister ID, which is used to interact with the wallet.

## Usage

### 1. Sending Tokens

To send tokens to another address, use the `send_tokens` function. Replace `<recipient_address>` with the desired recipient's address and `<amount>` with the number of tokens to send.

<!-- 
dfx canister call <canister_id> send_tokens '("<recipient_address>", <amount>)'
-->

### 2. Receiving Tokens

To receive tokens and update the balance, use the `receive_tokens` function:

<!-- 
dfx canister call <canister_id> receive_tokens '("<sender_address>", <amount>)'
 -->

### 3. Checking the Balance

To check the current token balance of the wallet, use the `get_balance` function:


<!-- dfx canister call <canister_id> get_balance -->


This will return the current balance stored in the wallet.

## Testing

### Run Unit Tests

The project includes a suite of unit tests to validate the core functionalities of the token wallet. To run the tests, use the following command:


<!-- cargo test -->


The tests will cover scenarios like sending tokens, receiving tokens, and fetching balances.

## Project Structure

<!-- 
├── src
│   ├── canister.rs       # Main canister logic
│   ├── lib.rs            # Library entry point
│   └── tests.rs          # Unit tests
├── dfx.json              # DFX configuration file
├── Cargo.toml            # Rust project configuration file
├── README.md             # Project documentation
└── ... -->


## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements, bug fixes, or improvements.


## License

This project is licensed under the MIT License - see the LICENSE file for details.




### Notes:

<!-- Dependencies: Ensure you have the latest versions of Rust and DFX SDK to avoid compatibility issues.
Further Development: Consider adding more advanced features such as multi-signature support or a front-end interface for easier interaction.
Security: Review and enhance the security of the smart contract before using it in a production environment.
Documentation: Keep the documentation updated as you make changes to the project. -->
