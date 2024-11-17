-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    company_name VARCHAR,
    business_type VARCHAR,  -- 'supplier', 'buyer', 'both'
    contact_number VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Equipment Categories table
CREATE TABLE equipment_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description TEXT,
    parent_category_id INTEGER REFERENCES equipment_categories(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Equipment table (replaces products)
CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL REFERENCES equipment_categories(id),
    name VARCHAR NOT NULL,
    description TEXT,
    manufacturer VARCHAR NOT NULL,
    model_number VARCHAR NOT NULL,
    year_manufactured INTEGER,
    condition VARCHAR NOT NULL, -- 'new', 'used', 'refurbished'
    price NUMERIC(15,2) NOT NULL,
    stock_level INTEGER NOT NULL,
    specifications JSONB, -- Store detailed specs as JSON
    weight_kg NUMERIC(10,2),
    dimensions_cm VARCHAR, -- Format: "length x width x height"
    power_requirements VARCHAR,
    certification_info TEXT,
    warranty_info TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Equipment Images table
CREATE TABLE equipment_images (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    image_url VARCHAR NOT NULL,
    is_primary BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Orders table
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    status VARCHAR NOT NULL, -- 'pending', 'confirmed', 'shipped', 'delivered', 'cancelled'
    total_amount NUMERIC(15,2) NOT NULL,
    shipping_address TEXT NOT NULL,
    shipping_method VARCHAR NOT NULL,
    tracking_number VARCHAR,
    estimated_delivery_date DATE,
    special_instructions TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Order Items table
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES orders(id),
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    quantity INTEGER NOT NULL,
    price_at_time NUMERIC(15,2) NOT NULL,
    warranty_selected BOOLEAN DEFAULT false,
    special_requirements TEXT
);

-- Reviews table
CREATE TABLE reviews (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    usage_duration VARCHAR, -- How long they've used the equipment
    pros TEXT,
    cons TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Technical Documents table
CREATE TABLE technical_documents (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    document_type VARCHAR NOT NULL, -- 'manual', 'specification', 'certification', etc.
    document_url VARCHAR NOT NULL,
    version VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Maintenance Records table
CREATE TABLE maintenance_records (
    id SERIAL PRIMARY KEY,
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    service_date DATE NOT NULL,
    service_type VARCHAR NOT NULL,
    description TEXT,
    performed_by VARCHAR,
    next_service_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX idx_equipment_category ON equipment(category_id);
CREATE INDEX idx_equipment_manufacturer ON equipment(manufacturer);
CREATE INDEX idx_equipment_condition ON equipment(condition);
CREATE INDEX idx_orders_user ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_order_items_order ON order_items(order_id);
CREATE INDEX idx_reviews_equipment ON reviews(equipment_id);
CREATE INDEX idx_reviews_user ON reviews(user_id);
CREATE INDEX idx_technical_docs_equipment ON technical_documents(equipment_id);
