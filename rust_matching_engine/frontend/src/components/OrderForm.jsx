import { useState } from "react";

export default function OrderForm({ onSubmit }) {
  const [price, setPrice] = useState("");
  const [quantity, setQuantity] = useState("");
  const [side, setSide] = useState("Buy");

  const handleSubmit = (e) => {
    e.preventDefault();

    onSubmit({
      price: Number(price),
      quantity: Number(quantity),
      user_id: "frontend_user",
      side,
    });

    setPrice("");
    setQuantity("");
  };

  return (
    <form onSubmit={handleSubmit} style={{ marginBottom: "20px" }}>
      <input
        placeholder="Price"
        value={price}
        onChange={(e) => setPrice(e.target.value)}
      />
      <input
        placeholder="Quantity"
        value={quantity}
        onChange={(e) => setQuantity(e.target.value)}
      />

      <select value={side} onChange={(e) => setSide(e.target.value)}>
        <option value="Buy">Buy</option>
        <option value="Sell">Sell</option>
      </select>

      <button type="submit">Place Order</button>
    </form>
  );
}
