import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import AssetList from './pages/AssetList';
import AssetDetail from './pages/AssetDetail';
import Header from './components/Header';

function App() {
  return (
    <Router>
      <Header />
      <Routes>
        <Route path="/" element={<AssetList />} />
        <Route path="/assets/:id" element={<AssetDetail />} />
        {/* Other routes can be added here */}
      </Routes>
    </Router>
  );
}

export default App; 