import React from 'react';

interface FilterControlsProps {
  onFilterChange: (filters: { sortBy?: string }) => void;
}

function FilterControls({ onFilterChange }: FilterControlsProps) {
  const handleFilterChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const { value } = event.target;
    onFilterChange({ sortBy: value });
  };

  return (
    <div className="filter-controls">
      <label htmlFor="sort">Sort By:</label>
      <select id="sort" onChange={handleFilterChange}>
        <option value="price">Price</option>
        <option value="stock">Stock</option>
        {/* ... other options ... */}
      </select>
    </div>
  );
}

export default FilterControls; 