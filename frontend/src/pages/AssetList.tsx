import React, { useEffect, useState } from 'react';
import api from '../services/api';

interface Asset {
  id: number;
  name: string;
  price: number;
  imageUrl: string;
  stock: number;
}

function AssetList() {
  const [assets, setAssets] = useState<Asset[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    const fetchAssets = async () => {
      try {
        console.log('Fetching assets...');
        const response = await api.get<Asset[]>('/api/assets');
        console.log('Assets received:', response.data);
        setAssets(response.data);
        setLoading(false);
      } catch (error: any) {
        console.error('Detailed error:', error);
        const errorMessage = error.response?.data?.message || error.message || 'Failed to load assets';
        setError(`Error: ${errorMessage}`);
        setLoading(false);
      }
    };

    fetchAssets();
  }, []);

  if (loading) {
    return <p>Loading assets...</p>;
  }

  if (error) {
    return (
      <div style={{ padding: '20px', color: 'red' }}>
        <h3>Error Loading Assets</h3>
        <p>{error}</p>
        <p>Please make sure the backend server is running at {process.env.REACT_APP_API_URL || 'http://localhost:8000'}</p>
      </div>
    );
  }

  return (
    <div>
      <h2>Marketplace Assets</h2>
      <div className="asset-list">
        {assets.length === 0 ? (
          <p>No assets available.</p>
        ) : (
          assets.map((asset) => (
            <div key={asset.id} className="asset-card">
              <h3>{asset.name}</h3>
              <p>Price: ${asset.price}</p>
              <p>Stock: {asset.stock}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default AssetList; 