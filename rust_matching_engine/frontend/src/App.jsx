import { useEffect, useState } from "react";
import { fetchDepth, fetchTicker, createOrder } from "./api";
import Ticker from "./components/Ticker";
import DepthTable from "./components/DepthTable";
import OrderForm from "./components/OrderForm";
import TradeChart from "./components/TradeChart";

function App() {
  const [flash, setFlash] = useState(false);
  const [trades, setTrades] = useState([]);
  const [bids, setBids] = useState([]);
  const [asks, setAsks] = useState([]);
  const [bestBid, setBestBid] = useState(null);
  const [bestAsk, setBestAsk] = useState(null);
  const [execution, setExecution] = useState(null);

  // ---------------------------
  // Derived Metrics
  // ---------------------------
  const spread =
    bestBid !== null && bestAsk !== null
      ? bestAsk - bestBid
      : null;

  const mid =
    bestBid !== null && bestAsk !== null
      ? (bestAsk + bestBid) / 2
      : null;

  // ---------------------------
  // Initial REST Load
  // ---------------------------
  const loadData = async () => {
    try {
      const depthRes = await fetchDepth();
      setBids(depthRes.data.bids);
      setAsks(depthRes.data.asks);

      const tickerRes = await fetchTicker();
      setBestBid(tickerRes.data.best_bid);
      setBestAsk(tickerRes.data.best_ask);
    } catch (err) {
      console.error("Initial load failed:", err);
    }
  };

  // ---------------------------
  // WebSocket Real-Time
  // ---------------------------
  useEffect(() => {
    const socket = new WebSocket("wss://solana-practice-production.up.railway.app/ws");

    socket.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.type === "market_update") {
        setBids(data.depth?.bids || []);
        setAsks(data.depth?.asks || []);
        setBestBid(data.ticker?.best_bid ?? null);
        setBestAsk(data.ticker?.best_ask ?? null);
        setTrades((data.trades || []).slice(-20).reverse());
      }
    };

    socket.onerror = (err) => {
      console.error("WebSocket error:", err);
    };

    return () => socket.close();
  }, []);

  useEffect(() => {
    loadData();
  }, []);

  // ---------------------------
  // Order Submission
  // ---------------------------
const handleOrder = async (data) => {
  try {
    const res = await createOrder(data);
    setExecution(res.data);

    if (res.data.filled_quantity > 0) {
      setFlash(true);
      setTimeout(() => setFlash(false), 300);
    }
  } catch (err) {
    console.error("Order failed:", err);
  }
};



  return (
    <div
      style={{
        background: "linear-gradient(135deg, #0d1117, #000814)",
        minHeight: "100vh",
        padding: "40px 20px",
        color: "white",
        fontFamily: "Inter, Arial",
      }}
    >
<div
  style={{
    maxWidth: "1250px",
    margin: "0 auto",
    backgroundColor: flash ? "#072f2f" : "#161b22",
    padding: "30px",
    borderRadius: "16px",
    boxShadow: flash
      ? "0 0 40px rgba(0,255,150,0.4)"
      : "0 0 40px rgba(0,255,150,0.05)",
    transition: "all 0.2s ease",
  }}
>

        <h1 style={{ marginBottom: "25px" }}>
          Rust Real-Time Matching Engine
        </h1>

        {/* Ticker */}
        <Ticker bestBid={bestBid} bestAsk={bestAsk} />

        {/* Spread + Mid */}
        <div
          style={{
            marginTop: "10px",
            marginBottom: "30px",
            color: "#aaa",
            fontSize: "14px",
          }}
        >
          Spread: {spread ?? "-"} | Mid: {mid ?? "-"}
        </div>

        {/* Price Chart */}
        <div style={{ marginBottom: "40px" }}>
    <h2 style={{ marginBottom: "10px" }}>Live Price Chart</h2>

<div
  style={{
    backgroundColor: "#0d1117",
    padding: "20px",
    borderRadius: "12px",
    height: "350px",        // IMPORTANT
    minHeight: "350px",     // IMPORTANT
  }}
>
  <TradeChart trades={trades} />
</div>


        </div>

        {/* Main Grid */}
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "1fr 1.2fr",
            gap: "40px",
          }}
        >
          {/* LEFT SIDE */}
          <div>
            <h2>Place Order</h2>
            <OrderForm onSubmit={handleOrder} />

            {execution && (
              <div
                style={{
                  marginTop: "20px",
                  padding: "15px",
                  backgroundColor: "#0d1117",
                  borderRadius: "12px",
                  border: "1px solid #222",
                }}
              >
                <strong>Execution Report</strong>
                <div style={{ marginTop: "8px" }}>
                  Filled: {execution.filled_quantity}
                </div>
                <div>Remaining: {execution.remaining_quantity}</div>
                <div>Average Price: {execution.average_price}</div>
              </div>
            )}

            {/* Trade History */}
            <div style={{ marginTop: "40px" }}>
              <h2>Recent Trades</h2>
              <div
                style={{
                  backgroundColor: "#0d1117",
                  padding: "15px",
                  borderRadius: "12px",
                  maxHeight: "280px",
                  overflowY: "auto",
                  border: "1px solid #222",
                }}
              >
                {trades.length === 0 && (
                  <div style={{ color: "#888" }}>No trades yet</div>
                )}

                {trades.map((t, i) => (
                  <div
                    key={i}
                    style={{
                      display: "flex",
                      justifyContent: "space-between",
                      padding: "6px 0",
                      borderBottom: "1px solid #1f2937",
                      color: "#ccc",
                      fontSize: "14px",
                    }}
                  >
                    <span>{t.price}</span>
                    <span>{t.quantity}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* RIGHT SIDE */}
          <div>
            <h2>Order Book</h2>
            <div
              style={{
                backgroundColor: "#0d1117",
                padding: "15px",
                borderRadius: "12px",
                border: "1px solid #222",
              }}
            >
              <DepthTable bids={bids} asks={asks} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
