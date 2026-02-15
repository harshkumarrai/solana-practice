use std::collections::{BTreeMap, VecDeque};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::types::{CreateOrder, DeleteOrder, ExecutionReport, Side};

#[derive(Clone)]
pub struct OpenOrder {
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
    pub user_id: String,
    pub order_id: String,
    pub filled_quantity: u64,
}

#[derive(Clone, Serialize)]
pub struct Trade {
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u128,
}

pub struct Orderbook {
    pub bids: BTreeMap<u64, VecDeque<OpenOrder>>, // highest = next_back()
    pub asks: BTreeMap<u64, VecDeque<OpenOrder>>, // lowest = next()
    pub trades: Vec<Trade>,
    pub order_id_index: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Depth {
    pub price: u64,
    pub quantity: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DepthResponse {
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}

impl Default for Orderbook {
    fn default() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            trades: Vec::new(),
            order_id_index: 0,
        }
    }
}

impl Orderbook {
pub fn clear(&mut self) {
    self.bids.clear();
    self.asks.clear();
    self.trades.clear();
    self.order_id_index = 0;
}


    pub fn create_order(&mut self, order: CreateOrder) -> ExecutionReport {
        // Simulated engine latency
        thread::sleep(Duration::from_millis(2));

        let mut remaining_qty = order.quantity;
        let mut filled_qty = 0;
        let mut total_trade_value = 0;

        match order.side {
            // ================= BUY =================
            Side::Buy => {
                while remaining_qty > 0 {
                    let best_ask_price = match self.asks.iter().next() {
                        Some((price, _)) => *price,
                        None => break,
                    };

                    if best_ask_price > order.price {
                        break;
                    }

                    let mut remove_level = false;

                    if let Some(level_orders) = self.asks.get_mut(&best_ask_price) {
                        while remaining_qty > 0 {
                            let mut top_order = match level_orders.pop_front() {
                                Some(o) => o,
                                None => break,
                            };

                            let trade_qty = remaining_qty.min(top_order.quantity);

                            remaining_qty -= trade_qty;
                            top_order.quantity -= trade_qty;
                            filled_qty += trade_qty;
                            total_trade_value += trade_qty * best_ask_price;

                            let timestamp = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis();

                            self.trades.push(Trade {
                                price: best_ask_price,
                                quantity: trade_qty,
                                timestamp,
                            });

                            // Keep trade history bounded
                            if self.trades.len() > 1000 {
                                self.trades.remove(0);
                            }

                            if top_order.quantity > 0 {
                                level_orders.push_front(top_order);
                            }

                            if remaining_qty == 0 {
                                break;
                            }
                        }

                        if level_orders.is_empty() {
                            remove_level = true;
                        }
                    }

                    if remove_level {
                        self.asks.remove(&best_ask_price);
                    }
                }

                // Add remaining to book
                if remaining_qty > 0 {
                    let order_id = self.order_id_index.to_string();
                    self.order_id_index += 1;

                    let open_order = OpenOrder {
                        price: order.price,
                        quantity: remaining_qty,
                        side: Side::Buy,
                        user_id: order.user_id,
                        order_id,
                        filled_quantity: 0,
                    };

                    self.bids
                        .entry(order.price)
                        .or_default()
                        .push_back(open_order);
                }
            }

            // ================= SELL =================
            Side::Sell => {
                while remaining_qty > 0 {
                    let best_bid_price = match self.bids.iter().next_back() {
                        Some((price, _)) => *price,
                        None => break,
                    };

                    if best_bid_price < order.price {
                        break;
                    }

                    let mut remove_level = false;

                    if let Some(level_orders) = self.bids.get_mut(&best_bid_price) {
                        while remaining_qty > 0 {
                            let mut top_order = match level_orders.pop_front() {
                                Some(o) => o,
                                None => break,
                            };

                            let trade_qty = remaining_qty.min(top_order.quantity);

                            remaining_qty -= trade_qty;
                            top_order.quantity -= trade_qty;
                            filled_qty += trade_qty;
                            total_trade_value += trade_qty * best_bid_price;

                            let timestamp = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis();


                            let mut rng = rand::thread_rng();
let noise: i64 = rng.gen_range(-2..=2);
let adjusted_price = (best_bid_price as i64 + noise).max(1) as u64;

                            self.trades.push(Trade {
                                price: adjusted_price,
                                quantity: trade_qty,
                                timestamp,
                            });

                            if self.trades.len() > 1000 {
                                self.trades.remove(0);
                            }

                            if top_order.quantity > 0 {
                                level_orders.push_front(top_order);
                            }

                            if remaining_qty == 0 {
                                break;
                            }
                        }

                        if level_orders.is_empty() {
                            remove_level = true;
                        }
                    }

                    if remove_level {
                        self.bids.remove(&best_bid_price);
                    }
                }

                // Add remaining to book
                if remaining_qty > 0 {
                    let order_id = self.order_id_index.to_string();
                    self.order_id_index += 1;

                    let open_order = OpenOrder {
                        price: order.price,
                        quantity: remaining_qty,
                        side: Side::Sell,
                        user_id: order.user_id,
                        order_id,
                        filled_quantity: 0,
                    };

                    self.asks
                        .entry(order.price)
                        .or_default()
                        .push_back(open_order);
                }
            }
        }

        let avg_price = if filled_qty > 0 {
            total_trade_value / filled_qty
        } else {
            0
        };

        ExecutionReport {
            filled_quantity: filled_qty,
            remaining_quantity: remaining_qty,
            average_price: avg_price,
        }
    }

    pub fn delete_order(&mut self, order: DeleteOrder) {
        for book in [&mut self.bids, &mut self.asks] {
            let mut target_price = None;

            for (price, orders) in book.iter() {
                if orders.iter().any(|o| o.order_id == order.order_id) {
                    target_price = Some(*price);
                    break;
                }
            }

            if let Some(price) = target_price {
                if let Some(orders) = book.get_mut(&price) {
                    orders.retain(|o| o.order_id != order.order_id);
                    if orders.is_empty() {
                        book.remove(&price);
                    }
                }
            }
        }
    }

    pub fn get_depth(&self) -> DepthResponse {
        let bids = self
            .bids
            .iter()
            .map(|(price, orders)| Depth {
                price: *price,
                quantity: orders.iter().map(|o| o.quantity).sum(),
            })
            .collect();

        let asks = self
            .asks
            .iter()
            .map(|(price, orders)| Depth {
                price: *price,
                quantity: orders.iter().map(|o| o.quantity).sum(),
            })
            .collect();

        DepthResponse { bids, asks }
    }

    pub fn get_ticker(&self) -> (Option<u64>, Option<u64>) {
        let best_bid = self.bids.iter().next_back().map(|(p, _)| *p);
        let best_ask = self.asks.iter().next().map(|(p, _)| *p);
        (best_bid, best_ask)
    }

    pub fn get_trades(&self) -> Vec<Trade> {
        self.trades.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CreateOrder, Side};

    #[test]
    fn test_partial_fill() {
        let mut ob = Orderbook::default();

        ob.create_order(CreateOrder {
            price: 100,
            quantity: 10,
            user_id: "seller".into(),
            side: Side::Sell,
        });

        let report = ob.create_order(CreateOrder {
            price: 100,
            quantity: 5,
            user_id: "buyer".into(),
            side: Side::Buy,
        });

        assert_eq!(report.filled_quantity, 5);
        assert_eq!(report.remaining_quantity, 0);
        assert_eq!(report.average_price, 100);
        assert_eq!(ob.trades.len(), 1);
    }
}
