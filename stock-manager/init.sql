CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    sku VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS stock_items (
    id UUID PRIMARY KEY,
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    location VARCHAR(255) NOT NULL,
    unit_cost DECIMAL(10, 2) NOT NULL CHECK (unit_cost >= 0),
    last_restocked TIMESTAMPTZ NOT NULL,
    UNIQUE (product_id, location)
);

CREATE INDEX IF NOT EXISTS idx_stock_items_product_id ON stock_items(product_id);
