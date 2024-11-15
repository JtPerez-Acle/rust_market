import React, { useState } from 'react';
import { buyAsset, sellAsset, updateStock, updatePrice } from '../services/api';

interface AssetCardProps {
  id: number;
  name: string;
  price: number;
  imageUrl: string;
  stock: number;
  description?: string;
  lastUpdated?: string;
}

function AssetCard({ id, name, price, imageUrl, stock, description, lastUpdated }: AssetCardProps) {
  const [message, setMessage] = useState<string>('');
  const [quantity, setQuantity] = useState<number>(1);
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [newPrice, setNewPrice] = useState<number>(price);
  const [newStock, setNewStock] = useState<number>(stock);

  const handleBuy = async () => {
    try {
      await buyAsset(id, quantity);
      setMessage(`Successfully purchased ${quantity} ${name}!`);
      setQuantity(1);
    } catch (error) {
      console.error('Error buying asset:', error);
      setMessage('Purchase failed.');
    }
  };

  const handleSell = async () => {
    try {
      await sellAsset(id, quantity);
      setMessage(`Successfully sold ${quantity} ${name}!`);
      setQuantity(1);
    } catch (error) {
      console.error('Error selling asset:', error);
      setMessage('Sale failed.');
    }
  };

  const handleUpdatePrice = async () => {
    try {
      await updatePrice(id, newPrice);
      setMessage('Price updated successfully!');
      setIsEditing(false);
    } catch (error) {
      console.error('Error updating price:', error);
      setMessage('Failed to update price.');
    }
  };

  const handleUpdateStock = async () => {
    try {
      await updateStock(id, newStock);
      setMessage('Stock updated successfully!');
      setIsEditing(false);
    } catch (error) {
      console.error('Error updating stock:', error);
      setMessage('Failed to update stock.');
    }
  };

  return (
    <div className="asset-card">
      <img src={imageUrl} alt={name} />
      <div className="asset-info">
        <h3>{name}</h3>
        {description && <p className="description">{description}</p>}
        
        <div className="price-section">
          <p>Price: ${price}</p>
          {isEditing && (
            <div className="edit-controls">
              <input
                type="number"
                value={newPrice}
                onChange={(e) => setNewPrice(Number(e.target.value))}
                min="0"
                step="0.01"
              />
              <button onClick={handleUpdatePrice}>Update Price</button>
            </div>
          )}
        </div>

        <div className="stock-section">
          <p>Stock: {stock}</p>
          {isEditing && (
            <div className="edit-controls">
              <input
                type="number"
                value={newStock}
                onChange={(e) => setNewStock(Number(e.target.value))}
                min="0"
              />
              <button onClick={handleUpdateStock}>Update Stock</button>
            </div>
          )}
        </div>

        <div className="transaction-controls">
          <input
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(Number(e.target.value))}
            min="1"
            max={stock}
          />
          <button onClick={handleBuy} disabled={stock < quantity}>Buy</button>
          <button onClick={handleSell}>Sell</button>
        </div>

        <button 
          className="edit-toggle"
          onClick={() => setIsEditing(!isEditing)}
        >
          {isEditing ? 'Cancel Editing' : 'Edit Asset'}
        </button>

        {lastUpdated && (
          <p className="last-updated">Last updated: {new Date(lastUpdated).toLocaleString()}</p>
        )}

        {message && <p className="message">{message}</p>}
      </div>
    </div>
  );
}

export default AssetCard; 