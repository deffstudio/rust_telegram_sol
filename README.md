# Telegram Solana Crypto Trading Bot

This project is a prototype for a Telegram bot that allows users to interact with the Solana blockchain and perform basic trading operations.

## Features

* **Connect to Local Solana Validator**: The bot can connect to your local Solana validator for testing and development purposes.
* **Check SOL Balance**: Users can query their SOL balance using the `/balance` command.
* **Telegram Integration**: The bot interacts with users through Telegram commands and messages.

## Getting Started

### Prerequisites

* **Rust and Cargo**: Make sure you have Rust and Cargo installed on your system. You can follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
* **Solana CLI**: Install the Solana CLI to generate and manage your keypair. Follow the instructions at [https://docs.solana.com/cli/install-solana-cli-tools](https://docs.solana.com/cli/install-solana-cli-tools).
* **Local Solana Validator**: Set up and run a local Solana validator for testing. You can use the `solana-test-validator` command.

### Installation

1. **Clone the repository**:

   ```bash
   git clone [https://github.com/deffstudio/rust_telegram_sol.git](https://github.com/deffstudio/rust_telegram_sol.git) 
   cd telegram-solana-trader

2. **Set Up Your Environment:**:
  ``` bash
  Create a .env file:

In the root directory of your project, create a file named .env.

Open the .env file and add the following line, replacing <YOUR_TELEGRAM_BOT_TOKEN> with the actual token you obtained from BotFather:

    ```bash
    TELOXIDE_TOKEN=<YOUR_TELEGRAM_BOT_TOKEN>
    Generate a Solana Keypair:

Open your terminal and use the Solana CLI to generate a new keypair:

    ```bash
    solana-keygen new --outfile path/to/your/keypair.json
    Use code with caution.

    This command will create a new keypair and save it to the specified file (keypair.json). Make sure to store this file securely, as it contains your private key, which is essential for interacting with the Solana blockchain.