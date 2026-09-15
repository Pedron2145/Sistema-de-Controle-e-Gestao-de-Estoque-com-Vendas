USE erp_basic;

CREATE TABLE IF NOT EXISTS sale_items (
  id INT AUTO_INCREMENT PRIMARY KEY,
  sale_id INT NOT NULL,
  product_id INT NOT NULL,
  product_name VARCHAR(160) NOT NULL,
  quantity INT NOT NULL,
  CONSTRAINT fk_sale_items_sale FOREIGN KEY (sale_id) REFERENCES sales(id) ON DELETE CASCADE,
  CONSTRAINT fk_sale_items_product FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE RESTRICT,
  UNIQUE KEY uq_sale_product (sale_id, product_id)
);

SET @has_legacy_product_id = (
  SELECT COUNT(*) FROM information_schema.COLUMNS
  WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'sales' AND COLUMN_NAME = 'product_id'
);
SET @copy_legacy_items = IF(
  @has_legacy_product_id > 0,
  'INSERT IGNORE INTO sale_items (sale_id, product_id, product_name, quantity) SELECT id, product_id, product_name, quantity FROM sales',
  'SELECT 1'
);
PREPARE copy_statement FROM @copy_legacy_items;
EXECUTE copy_statement;
DEALLOCATE PREPARE copy_statement;

SET @drop_legacy_foreign_key = IF(
  @has_legacy_product_id > 0,
  'ALTER TABLE sales DROP FOREIGN KEY fk_sales_product',
  'SELECT 1'
);
PREPARE drop_fk_statement FROM @drop_legacy_foreign_key;
EXECUTE drop_fk_statement;
DEALLOCATE PREPARE drop_fk_statement;

SET @drop_legacy_columns = IF(
  @has_legacy_product_id > 0,
  'ALTER TABLE sales DROP COLUMN product_id, DROP COLUMN product_name, DROP COLUMN quantity',
  'SELECT 1'
);
PREPARE drop_columns_statement FROM @drop_legacy_columns;
EXECUTE drop_columns_statement;
DEALLOCATE PREPARE drop_columns_statement;
