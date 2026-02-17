# Anchor Escrow Program

A simple Solana smart contract built with Anchor that implements a basic escrow mechanism for SPL token swaps.

## Overview

This program enables trustless token swaps between two parties:

### Maker
- Initializes an escrow PDA.
- Deposits **Token A** into a vault.
- Specifies the amount of **Token B** they expect in return.

### Taker
- Accepts the deal by:
  - Depositing the specified amount of **Token B** to the maker.
  - Receiving **Token A** from the vault.
- The escrow and vault accounts are closed after completion.

### Refund (Maker)
If no taker accepts the deal:
- The maker can withdraw **Token A**.
- The escrow and vault accounts are closed.

---

## Features

- Uses **Program Derived Addresses (PDAs)** for secure escrow management.
- Fully supports **SPL tokens**.
- Automatic closing of escrow and vault accounts on completion or refund.
- Integration tests included.

---

## Prerequisites

Make sure the following tools are installed:

- **Anchor CLI** (v0.32.1 or later)  
  Installed via AVM.

- **Surfpool CLI**  
  For enhanced local testing and runbooks.

 
