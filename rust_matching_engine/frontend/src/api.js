import axios from "axios";

const API = "http://127.0.0.1:8080";

export const fetchDepth = () => axios.get(`${API}/depth`);
export const fetchTicker = () => axios.get(`${API}/ticker`);
export const createOrder = (data) =>
  axios.post(`${API}/order`, data);
