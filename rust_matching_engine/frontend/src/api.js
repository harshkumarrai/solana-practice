import axios from "axios";

const BASE_URL = "https://solana-practice-production.up.railway.app";

export const fetchDepth = () => axios.get(`${BASE_URL}/depth`);
export const fetchTicker = () => axios.get(`${BASE_URL}/ticker`);
export const createOrder = (data) =>
  axios.post(`${BASE_URL}/order`, data);
