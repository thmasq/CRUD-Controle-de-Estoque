INSERT INTO products (id, name, description, sku, created_at, updated_at)
VALUES
    ('f47ac10b-58cc-4372-a567-0e02b2c3d479', 'Laptop', 'High-performance laptop', 'LT-001', NOW(), NOW()),
    ('38c5d13c-4fc2-4ae1-8fdb-24198d9f5f7f', 'Mouse', 'Wireless mouse', 'MS-002', NOW(), NOW()),
    ('9a48ad74-4a35-4a69-8a3b-6c9f5cd8b1b4', 'Keyboard', 'Mechanical keyboard', 'KB-003', NOW(), NOW()),
    ('3e5d0870-26d5-42a2-8d74-e7e4c5e7b138', 'Monitor', '27-inch 4K display', 'MN-004', NOW(), NOW()),
    ('c37fd16a-5f7c-4e4b-972a-5f1e2af7c8a7', 'Headphones', 'Noise-cancelling headphones', 'HP-005', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO stock_items (id, product_id, quantity, location, unit_cost, last_restocked)
VALUES
    ('b5e2a896-8c58-44a3-a160-d10876ba53b4', 'f47ac10b-58cc-4372-a567-0e02b2c3d479', 10, 'Warehouse A', 999.99, NOW()),
    ('f8a5d246-6cd9-4fbd-9fd9-0bfd71c9e918', '38c5d13c-4fc2-4ae1-8fdb-24198d9f5f7f', 20, 'Warehouse A', 24.99, NOW()),
    ('a9d8c521-35a9-4c76-a142-0a0bc3e89c71', '9a48ad74-4a35-4a69-8a3b-6c9f5cd8b1b4', 15, 'Warehouse B', 89.99, NOW()),
    ('c28f2df6-9a5b-46a7-8c4d-7f9e5c2c3a8d', '3e5d0870-26d5-42a2-8d74-e7e4c5e7b138', 5, 'Warehouse A', 349.99, NOW()),
    ('e2a6d8f9-4b7c-43a5-9e8d-1f2c3b4a5d6e', 'c37fd16a-5f7c-4e4b-972a-5f1e2af7c8a7', 25, 'Warehouse C', 129.99, NOW())
ON CONFLICT (id) DO NOTHING;
