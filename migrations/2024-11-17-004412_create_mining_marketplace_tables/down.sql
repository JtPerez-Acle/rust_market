-- This file should undo anything in `up.sql`

-- Drop indexes
DROP INDEX IF EXISTS idx_technical_docs_equipment;
DROP INDEX IF EXISTS idx_reviews_user;
DROP INDEX IF EXISTS idx_reviews_equipment;
DROP INDEX IF EXISTS idx_order_items_order;
DROP INDEX IF EXISTS idx_orders_status;
DROP INDEX IF EXISTS idx_orders_user;
DROP INDEX IF EXISTS idx_equipment_condition;
DROP INDEX IF EXISTS idx_equipment_manufacturer;
DROP INDEX IF EXISTS idx_equipment_category;

-- Drop tables in reverse order of creation (respecting foreign key constraints)
DROP TABLE IF EXISTS maintenance_records;
DROP TABLE IF EXISTS technical_documents;
DROP TABLE IF EXISTS reviews;
DROP TABLE IF EXISTS order_items;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS equipment_images;
DROP TABLE IF EXISTS equipment;
DROP TABLE IF EXISTS equipment_categories;
DROP TABLE IF EXISTS users;
