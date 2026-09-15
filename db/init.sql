CREATE DATABASE IF NOT EXISTS erp_basic;
USE erp_basic;

CREATE TABLE IF NOT EXISTS users (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(120) NOT NULL,
  email VARCHAR(160) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  role ENUM('admin', 'vendedor', 'estoquista', 'pce') NOT NULL DEFAULT 'vendedor',
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_permissions (
  id INT AUTO_INCREMENT PRIMARY KEY,
  user_id INT NOT NULL,
  module_name VARCHAR(80) NOT NULL,
  can_view BOOLEAN NOT NULL DEFAULT FALSE,
  can_create BOOLEAN NOT NULL DEFAULT FALSE,
  can_edit BOOLEAN NOT NULL DEFAULT FALSE,
  can_delete BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY uq_user_module (user_id, module_name),
  CONSTRAINT fk_user_permissions_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sessions (
  id INT AUTO_INCREMENT PRIMARY KEY,
  user_id INT NOT NULL,
  token VARCHAR(255) NOT NULL UNIQUE,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_sessions_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS products (
  id INT AUTO_INCREMENT PRIMARY KEY,
  code VARCHAR(40) NOT NULL,
  name VARCHAR(160) NOT NULL,
  manufacturer VARCHAR(160) NOT NULL,
  brand VARCHAR(160) NOT NULL,
  supplier VARCHAR(160) NOT NULL,
  status ENUM('ativo', 'inativo') NOT NULL DEFAULT 'ativo',
  street SMALLINT NOT NULL DEFAULT 0,
  position SMALLINT NOT NULL DEFAULT 0,
  level SMALLINT NOT NULL DEFAULT 0,
  apartment SMALLINT NOT NULL DEFAULT 0,
  quantity INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX idx_products_search (code, status, supplier),
  INDEX idx_products_location (street, position, level, apartment)
);

CREATE TABLE IF NOT EXISTS sales (
  id INT AUTO_INCREMENT PRIMARY KEY,
  client_name VARCHAR(160) NOT NULL,
  customer_type ENUM('Pessoa física', 'Empresa') NOT NULL DEFAULT 'Pessoa física',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

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

CREATE TABLE IF NOT EXISTS picking_demands (
  id INT AUTO_INCREMENT PRIMARY KEY,
  sale_id INT NOT NULL UNIQUE,
  status ENUM('aberta', 'concluida') NOT NULL DEFAULT 'aberta',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  completed_at TIMESTAMP NULL,
  CONSTRAINT fk_demands_sale FOREIGN KEY (sale_id) REFERENCES sales(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS picking_demand_items (
  id INT AUTO_INCREMENT PRIMARY KEY,
  demand_id INT NOT NULL,
  product_id INT NOT NULL,
  product_code VARCHAR(40) NOT NULL,
  product_name VARCHAR(160) NOT NULL,
  street SMALLINT NOT NULL,
  position SMALLINT NOT NULL,
  level SMALLINT NOT NULL,
  apartment SMALLINT NOT NULL,
  quantity INT NOT NULL,
  CONSTRAINT fk_demand_items_demand FOREIGN KEY (demand_id) REFERENCES picking_demands(id) ON DELETE CASCADE,
  CONSTRAINT fk_demand_items_product FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE RESTRICT
);

ALTER TABLE users MODIFY role ENUM('admin', 'vendedor', 'estoquista', 'pce') NOT NULL DEFAULT 'vendedor';

UPDATE products SET code = CONCAT('LEGACY-', id) WHERE code = '';
UPDATE products SET supplier = manufacturer WHERE supplier = '';
