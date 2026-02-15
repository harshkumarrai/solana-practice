import { useMemo } from "react";
import {
  ComposedChart,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  Bar,
  Line,
} from "recharts";

function TradeChart({ trades }) {
  const candles = useMemo(() => {
    if (!trades || trades.length === 0) return [];

    const bucketSize = 5000; // 5 sec aggregation
    const buckets = {};

    trades.forEach((trade) => {
      if (!trade.timestamp) return;

      const bucket =
        Math.floor(trade.timestamp / bucketSize) * bucketSize;

      if (!buckets[bucket]) {
        buckets[bucket] = {
          time: bucket,
          open: trade.price,
          high: trade.price,
          low: trade.price,
          close: trade.price,
        };
      } else {
        buckets[bucket].high = Math.max(
          buckets[bucket].high,
          trade.price
        );
        buckets[bucket].low = Math.min(
          buckets[bucket].low,
          trade.price
        );
        buckets[bucket].close = trade.price;
      }
    });

    return Object.values(buckets).sort((a, b) => a.time - b.time);
  }, [trades]);

  if (!candles.length) return null;

  const minPrice = Math.min(...candles.map(c => c.low));
  const maxPrice = Math.max(...candles.map(c => c.high));

  return (
    <div style={{ height: 350 }}>
      <ResponsiveContainer width="100%" height="100%">
        <ComposedChart data={candles}>
          <XAxis dataKey="time" hide />
          <YAxis
            domain={[
              minPrice * 0.995,
              maxPrice * 1.005
            ]}
          />
          <Tooltip />

          {/* Wick */}
          <Line
            type="monotone"
            dataKey="high"
            stroke="#888"
            dot={false}
          />
          <Line
            type="monotone"
            dataKey="low"
            stroke="#888"
            dot={false}
          />

          {/* Candle body */}
          <Bar
            dataKey="close"
            fill="#00ff99"
          />
        </ComposedChart>
      </ResponsiveContainer>
    </div>
  );
}

export default TradeChart;
