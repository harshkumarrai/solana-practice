export default function Ticker({ bestBid, bestAsk }) {
  return (
    <div style={{
      display: "flex",
      justifyContent: "space-between",
      padding: "10px",
      background: "#111",
      color: "white",
      fontWeight: "bold"
    }}>
      <div style={{ color: "green" }}>
        Best Bid: {bestBid ?? "-"}
      </div>
      <div style={{ color: "red" }}>
        Best Ask: {bestAsk ?? "-"}
      </div>
    </div>
  );
}
