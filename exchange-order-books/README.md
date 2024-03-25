# Exchange Order Book Simulation

This project is a simple simulation of an exchange order book and matching engine using Typescript.

## Overview

The goal of this project is to create a simple order book for a mock cryptocurrency exchange. The order book should be able to match buy and sell orders based on price and time priority.

## Features

The application should have the following features:

1. Create an Order Book: The order book should hold buy and sell orders. Each order should have the following properties:
   - Order ID
   - Price
   - Quantity
   - Buy/Sell indicator
   - Timestamp
2. Place Orders: Ability to place buy and sell orders into the order book. The orders should be sorted by price and time.
3. Match Orders: A mechanism to match the topmost buy and sell orders if the buy order price equals or is greater than the sell order price.
4. Displaying the Order Book: Functionality to show the current state of the order book with separate lists for buy and sell orders.
5. Testing: Test scenarios to validate the implemented functionalities.

## Project Structure

```
./
│
├─ src/
│ ├─ index.ts # Main application entry point
│ ├─ orderBook.ts # Main application entry point
│ └─ orderBook.test.ts # Unit test case for order book operations
└─ package.json # NPM package file
```

## How to Run

1. Install Dependencies
   `npm install`
2. To Run the Application
   `npm start`
3. To Run the Tests
   `npm test`
