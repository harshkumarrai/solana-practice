export default function DepthTable({ bids, asks }) {
  const sortedBids = [...bids].sort((a, b) => b.price - a.price);
  const sortedAsks = [...asks].sort((a, b) => a.price - b.price);

  const maxBidQty =
    sortedBids.length > 0
      ? Math.max(...sortedBids.map((b) => b.quantity))
      : 1;

  const maxAskQty =
    sortedAsks.length > 0
      ? Math.max(...sortedAsks.map((a) => a.quantity))
      : 1;

  return (
    <div style={{ display: "flex", gap: "40px" }}>
      {/* BIDS */}
      <div style={{ flex: 1 }}>
        <h3 style={{ color: "#00ff99", marginBottom: "10px" }}>Bids</h3>

        {sortedBids.map((b, i) => {
          const width = (b.quantity / maxBidQty) * 100;

          return (
            <div
              key={i}
              style={{
                position: "relative",
                padding: "6px 8px",
                marginBottom: "4px",
                borderRadius: "6px",
                overflow: "hidden",
                fontSize: "14px",
                transition: "all 0.3s ease",
              }}
            >
              {/* Depth Bar */}
              <div
                style={{
                  position: "absolute",
                  top: 0,
                  right: 0,
                  height: "100%",
                  width: `${width}%`,
                  backgroundColor: "rgba(0,255,153,0.15)",
                  transition: "width 0.3s ease",
                }}
              />

              <div
                style={{
                  position: "relative",
                  display: "flex",
                  justifyContent: "space-between",
                  color: "#00ff99",
                }}
              >
                <span>{b.price}</span>
                <span>{b.quantity}</span>
              </div>
            </div>
          );
        })}
      </div>

      {/* ASKS */}
      <div style={{ flex: 1 }}>
        <h3 style={{ color: "#ff4d4f", marginBottom: "10px" }}>Asks</h3>

        {sortedAsks.map((a, i) => {
          const width = (a.quantity / maxAskQty) * 100;

          return (
            <div
              key={i}
              style={{
                position: "relative",
                padding: "6px 8px",
                marginBottom: "4px",
                borderRadius: "6px",
                overflow: "hidden",
                fontSize: "14px",
                transition: "all 0.3s ease",
              }}
            >
              {/* Depth Bar */}
              <div
                style={{
                  position: "absolute",
                  top: 0,
                  left: 0,
                  height: "100%",
                  width: `${width}%`,
                  backgroundColor: "rgba(255,77,79,0.15)",
                  transition: "width 0.3s ease",
                }}
              />

              <div
                style={{
                  position: "relative",
                  display: "flex",
                  justifyContent: "space-between",
                  color: "#ff4d4f",
                }}
              >
                <span>{a.price}</span>
                <span>{a.quantity}</span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
