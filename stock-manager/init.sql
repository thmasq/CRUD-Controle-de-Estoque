CREATE TABLE categories (
  id UUID PRIMARY KEY,
  name VARCHAR(100) NOT NULL UNIQUE,
  description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE warehouses (
  id UUID PRIMARY KEY,
  name VARCHAR(100) NOT NULL UNIQUE,
  location TEXT NOT NULL,
  contact_info TEXT,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  sku VARCHAR(50) NOT NULL UNIQUE,
  category_id UUID REFERENCES categories(id),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE stock_items (
  id UUID PRIMARY KEY,
  product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  warehouse_id UUID NOT NULL REFERENCES warehouses(id),
  quantity INTEGER NOT NULL CHECK (quantity >= 0),
  unit_cost NUMERIC(10,2) NOT NULL CHECK (unit_cost >= 0),
  last_restocked TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  UNIQUE (product_id, warehouse_id)
);

CREATE TABLE stock_transactions (
  id UUID PRIMARY KEY,
  stock_item_id UUID NOT NULL REFERENCES stock_items(id),
  quantity INTEGER NOT NULL,
  transaction_type VARCHAR(20) NOT NULL CHECK (
    transaction_type IN ('IN', 'OUT', 'ADJUSTMENT')
  ),
  reference_number VARCHAR(100),
  notes TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_by VARCHAR(100) NOT NULL
);

CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_stock_items_product ON stock_items(product_id);
CREATE INDEX idx_stock_items_warehouse ON stock_items(warehouse_id);
CREATE INDEX idx_stock_transactions_stock_item ON stock_transactions(stock_item_id);
CREATE INDEX idx_stock_transactions_created_at ON stock_transactions(created_at);
