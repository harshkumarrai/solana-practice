import axios from "axios";

const BASE_URL = "https://solana-practice-production.up.railway.app";


export const fetchDepth = () => axios.get(`${API}/depth`);
export const fetchTicker = () => axios.get(`${API}/ticker`);
export const createOrder = (data) =>
  axios.post(`${API}/order`, data);
