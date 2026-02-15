# ⚡ Rust Real-Time Matching Engine

A high-performance, price-time priority matching engine built in **Rust**, with a real-time WebSocket-powered frontend and OHLC candlestick visualization.

This project simulates the core architecture of a centralized exchange (CEX) including:

* Limit order placement
* Price-time priority matching
* Partial fills
* Order book aggregation
* Real-time market data streaming
* OHLC candle generation

---

# 🚀 Features

### Matching Engine (Backend - Rust)

* BTreeMap-based order book (price sorted)
* FIFO execution within price level (VecDeque)
* Partial fill handling
* Trade generation with timestamps
* Execution reports
* Depth aggregation
* Best bid / best ask ticker
* Reset endpoint
* WebSocket broadcast of market updates

### Frontend (React + Recharts)

* Real-time order book updates
* Trade history panel
* Spread & mid-price display
* OHLC candle aggregation (5s buckets)
* Dark exchange-style UI

---

# 🏗 Architectural Design

The system follows a clean exchange-style architecture:

```
                ┌──────────────────────┐
                │      Frontend        │
                │  React + Recharts    │
                │                      │
                │  - Order Form        │
                │  - Order Book UI     │
                │  - Candlestick Chart │
                └───────────┬──────────┘
                            │ REST (HTTP)
                            │ WebSocket
                            ▼
                ┌──────────────────────┐
                │   API Layer (Actix)  │
                │                      │
                │  - /order            │
                │  - /depth            │
                │  - /ticker           │
                │  - /reset            │
                │  - /ws               │
                └───────────┬──────────┘
                            │
                            ▼
                ┌──────────────────────┐
                │   Matching Engine    │
                │      (Orderbook)     │
                │                      │
                │  Bids  : BTreeMap    │
                │  Asks  : BTreeMap    │
                │  FIFO  : VecDeque    │
                │  Trades: Vec<Trade>  │
                └──────────────────────┘
```

---

# ⚙️ Matching Logic

## Price-Time Priority

### BUY Order Flow

1. Match against lowest ask
2. Execute if ask <= buy price
3. Reduce quantities
4. Generate trade event
5. Continue until filled or no liquidity
6. If remaining → insert into bids

### SELL Order Flow

1. Match against highest bid
2. Execute if bid >= sell price
3. Reduce quantities
4. Generate trade event
5. Continue until filled or no liquidity
6. If remaining → insert into asks

---

# 📊 Market Data Flow

1. Order is submitted
2. Matching engine updates state
3. Trade is generated
4. Depth recalculated
5. Ticker recalculated
6. WebSocket broadcasts `market_update`
7. Frontend updates:

   * Order book
   * Trades panel
   * Candlestick chart

---

# 🕯 Candle Aggregation

Trades are bucketed into 5-second intervals.

For each bucket:

* Open  = first trade price
* High  = max trade price
* Low   = min trade price
* Close = last trade price

This creates realistic exchange-style OHLC candles.

---

# 📂 Project Structure

```
orderbook/     
 ├── orderbook.rs
 ├── routes.rs
 ├── main.rs
 └── types.rs

frontend/
 ├── App.jsx
 ├── components/
 │     ├── DepthTable.jsx
 │     ├── OrderForm.jsx
 │     ├── Ticker.jsx
 │     └── TradeChart.jsx
```

---

# ▶ Running the Project

## Backend

```
cargo run
```

Server runs on:

```
http://127.0.0.1:8080
```

## Frontend

```
npm install
npm run dev
```

---

:

🚀 Production Deployment
🌍 Live Architecture

Frontend deployed on Vercel

Backend (Rust + Actix Web) deployed on Railway

WebSocket (WSS) enabled for secure real-time streaming

Production API base URL configured via environment variables

CORS enabled for cross-origin communication

Railway service running on Port 8080

🔐 Production Configuration

BASE_URL switched from http://127.0.0.1:8080 →
https://solana-practice-production.up.railway.app

WebSocket upgraded from:

ws://localhost:8080/ws


to:

wss://solana-practice-production.up.railway.app/ws


Secure HTTPS + WSS enforced in production.

📡 Deployment Architecture Diagram
                ┌───────────────────┐
                │     User Browser  │
                └─────────┬─────────┘
                          │
                          ▼
                ┌───────────────────┐
                │  Vercel Frontend  │
                │  (React + Recharts)│
                └─────────┬─────────┘
                          │ HTTPS / WSS
                          ▼
                ┌───────────────────┐
                │ Railway Backend   │
                │ Rust + Actix Web  │
                │ Matching Engine   │
                └─────────┬─────────┘
                          │
                          ▼
                In-Memory Orderbook


# 📌 Why This Project Matters

This project demonstrates:

* Systems-level thinking (Rust data structures)
* Exchange microstructure understanding
* Real-time system design
* WebSocket streaming
* Market data engineering
* OHLC aggregation logic

It models the core engine behavior used in centralized exchanges.

---

# 🔮 Possible Extensions

* Market orders
* Stop-loss orders
* Order cancellation by ID
* Latency benchmarking
* Persistent storage (Postgres)
* Matching engine benchmarking
* Multi-symbol support
* Liquidity heatmap visualization

---

# 🧠 Built For

Developers interested in:

* Low-latency systems
* Trading infrastructure
* Blockchain exchange architecture
* High-performance Rust systems

---

# 📄 License

MIT License

---

**Rust • Real-Time Systems • Matching Engine Design**
