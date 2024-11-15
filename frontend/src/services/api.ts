import axios from 'axios';

const api = axios.create({
  baseURL: process.env.REACT_APP_API_URL || 'http://localhost:8000',
});

// Add request interceptor for debugging
api.interceptors.request.use(
  (config) => {
    console.log('API Request:', config.method?.toUpperCase(), config.url);
    return config;
  },
  (error) => {
    console.error('API Request Error:', error);
    return Promise.reject(error);
  }
);

// Add response interceptor for debugging
api.interceptors.response.use(
  (response) => {
    console.log('API Response:', response.status, response.data);
    return response;
  },
  (error) => {
    console.error('API Response Error:', {
      status: error.response?.status,
      data: error.response?.data,
      message: error.message,
    });
    return Promise.reject(error);
  }
);

export const buyAsset = (assetId: number, quantity = 1) => {
  return api.post(`/api/assets/${assetId}/buy`, { quantity });
};

export const sellAsset = (assetId: number, quantity = 1) => {
  return api.post(`/api/assets/${assetId}/sell`, { quantity });
};

export const updateStock = (assetId: number, quantity: number) => {
  return api.put(`/api/assets/${assetId}/stock`, { quantity });
};

export const updatePrice = (assetId: number, price: number) => {
  return api.put(`/api/assets/${assetId}/price`, { price });
};

export default api; 