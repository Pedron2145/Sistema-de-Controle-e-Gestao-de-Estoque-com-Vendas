use argon2::{
  password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
  id: i64,
  name: String,
  email: String,
  role: String,
  active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermission {
  module: String,
  can_view: bool,
  can_create: bool,
  can_edit: bool,
  can_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
  token: String,
  user: UserSummary,
  permissions: Vec<UserPermission>,
}

#[derive(Debug, Deserialize)]
pub struct FirstAdminPayload {
  name: String,
  email: String,
  password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
  email: String,
  password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
  name: String,
  email: String,
  password: String,
  role: String,
  session_token: String,
  permissions: Option<Vec<UserPermission>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionsPayload {
  session_token: String,
  user_id: i64,
  permissions: Vec<UserPermission>,
}

#[derive(Debug, Serialize)]
pub struct AppStatus {
  needs_first_admin: bool,
  users_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Product {
  id: i64,
  code: String,
  name: String,
  manufacturer: String,
  brand: String,
  supplier: String,
  status: String,
  street: i64,
  position: i64,
  level: i64,
  apartment: i64,
  quantity: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Sale {
  id: i64,
  client_name: String,
  customer_type: String,
  item_count: i64,
  date: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductPayload {
  code: String,
  name: String,
  manufacturer: String,
  brand: String,
  supplier: String,
  status: String,
  street: i64,
  position: i64,
  level: i64,
  apartment: i64,
  quantity: i64,
  session_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductPayload {
  id: i64,
  code: String,
  name: String,
  manufacturer: String,
  brand: String,
  supplier: String,
  status: String,
  street: i64,
  position: i64,
  level: i64,
  apartment: i64,
  quantity: i64,
  session_token: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteProductPayload {
  id: i64,
  session_token: String,
}

#[derive(Debug, Deserialize)]
pub struct SalePayload {
  items: Vec<SaleItemPayload>,
  client_name: String,
  customer_type: String,
  session_token: String,
}

#[derive(Debug, Deserialize)]
pub struct SaleItemPayload {
  product_id: i64,
  quantity: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PickingDemand {
  id: i64,
  sale_id: i64,
  status: String,
  created_at: String,
  client_name: String,
  items: Vec<PickingDemandItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PickingDemandItem {
  code: String,
  name: String,
  quantity: i64,
  position: String,
}

#[derive(Debug, Deserialize)]
pub struct DemandStatusPayload {
  demand_id: i64,
  session_token: String,
}

fn database_url() -> String {
  env::var("DATABASE_URL").unwrap_or_else(|_| {
    "mysql://erp_app:erp_app_pass@127.0.0.1:3306/erp_basic".to_string()
  })
}

async fn get_pool() -> Result<MySqlPool, String> {
  MySqlPoolOptions::new()
    .max_connections(10)
    .connect(&database_url())
    .await
    .map_err(|error| format!("Não foi possível conectar ao banco: {error}"))
}

fn hash_password(password: &str) -> Result<String, String> {
  let salt = SaltString::generate(rand::rngs::OsRng);
  let argon2 = Argon2::default();
  argon2
    .hash_password(password.as_bytes(), &salt)
    .map(|hash| hash.to_string())
    .map_err(|error| format!("Falha ao gerar hash do password: {error}"))
}

fn verify_password(password: &str, hashed_password: &str) -> Result<bool, String> {
  let parsed_hash = PasswordHash::new(hashed_password)
    .map_err(|error| format!("Hash inválido: {error}"))?;
  Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash)
    .map(|_| true)
    .or(Ok(false))
}

fn session_token_for(user_id: i64) -> String {
  let nanos = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_nanos();
  format!("erp_{user_id}_{nanos}")
}

fn user_from_row(row: &sqlx::mysql::MySqlRow) -> UserSummary {
  UserSummary {
    id: row.get("id"),
    name: row.get("name"),
    email: row.get("email"),
    role: row.get("role"),
    active: row.get("active"),
  }
}

fn default_permissions_for_role(role: &str) -> Vec<UserPermission> {
  let modules = ["vendas", "estoque", "demandas", "relatorios", "clientes"];

  if role == "admin" {
    return modules
      .iter()
      .map(|module| UserPermission {
        module: module.to_string(),
        can_view: true,
        can_create: true,
        can_edit: true,
        can_delete: true,
      })
      .collect();
  }

  modules
    .iter()
    .map(|module| match (role, *module) {
      ("vendedor", "vendas") => UserPermission {
        module: module.to_string(), can_view: true, can_create: true, can_edit: false, can_delete: false,
      },
      ("vendedor", "estoque") => UserPermission {
        module: module.to_string(), can_view: true, can_create: false, can_edit: false, can_delete: false,
      },
      ("estoquista", "estoque") => UserPermission {
        module: module.to_string(), can_view: true, can_create: false, can_edit: false, can_delete: false,
      },
      ("estoquista", "demandas") => UserPermission {
        module: module.to_string(), can_view: true, can_create: false, can_edit: true, can_delete: false,
      },
      ("pce", "estoque") => UserPermission {
        module: module.to_string(), can_view: true, can_create: true, can_edit: true, can_delete: false,
      },
      ("pce", "relatorios") => UserPermission {
        module: module.to_string(), can_view: true, can_create: true, can_edit: false, can_delete: false,
      },
      ("vendedor", "relatorios") | ("estoquista", "relatorios") => UserPermission {
        module: module.to_string(), can_view: false, can_create: false, can_edit: false, can_delete: false,
      },
      (_, "vendas") => UserPermission {
        module: module.to_string(),
        can_view: true,
        can_create: true,
        can_edit: false,
        can_delete: false,
      },
      (_, "estoque") => UserPermission {
        module: module.to_string(),
        can_view: true,
        can_create: false,
        can_edit: false,
        can_delete: false,
      },
      (_, "relatorios") => UserPermission {
        module: module.to_string(),
        can_view: true,
        can_create: false,
        can_edit: false,
        can_delete: false,
      },
      (_, "clientes") => UserPermission {
        module: module.to_string(),
        can_view: true,
        can_create: true,
        can_edit: false,
        can_delete: false,
      },
      _ => UserPermission {
        module: module.to_string(),
        can_view: false,
        can_create: false,
        can_edit: false,
        can_delete: false,
      },
    })
    .collect()
}

async fn list_permissions_for_user(pool: &MySqlPool, user_id: i64) -> Result<Vec<UserPermission>, String> {
  let rows = sqlx::query(
    "SELECT module_name, can_view, can_create, can_edit, can_delete FROM user_permissions WHERE user_id = ? ORDER BY module_name ASC",
  )
  .bind(user_id)
  .fetch_all(pool)
  .await
  .map_err(|error| format!("Falha ao consultar permissões: {error}"))?;

  let mut permissions = Vec::new();
  for row in rows {
    permissions.push(UserPermission {
      module: row.get("module_name"),
      can_view: row.get("can_view"),
      can_create: row.get("can_create"),
      can_edit: row.get("can_edit"),
      can_delete: row.get("can_delete"),
    });
  }

  if permissions.is_empty() {
    let role: String = sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
      .bind(user_id)
      .fetch_one(pool)
      .await
      .map_err(|error| format!("Falha ao buscar perfil do usuário: {error}"))?;

    let defaults = default_permissions_for_role(&role);
    for permission in &defaults {
      sqlx::query(
        "INSERT INTO user_permissions (user_id, module_name, can_view, can_create, can_edit, can_delete) VALUES (?, ?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE can_view = VALUES(can_view), can_create = VALUES(can_create), can_edit = VALUES(can_edit), can_delete = VALUES(can_delete)",
      )
      .bind(user_id)
      .bind(&permission.module)
      .bind(permission.can_view)
      .bind(permission.can_create)
      .bind(permission.can_edit)
      .bind(permission.can_delete)
      .execute(pool)
      .await
      .map_err(|error| format!("Falha ao salvar permissões padrões: {error}"))?;
    }

    return Ok(defaults);
  }

  Ok(permissions)
}

async fn require_admin_session(pool: &MySqlPool, session_token: &str) -> Result<UserSummary, String> {
  let row = sqlx::query(
    "SELECT u.id, u.name, u.email, u.role, u.active FROM sessions s INNER JOIN users u ON u.id = s.user_id WHERE s.token = ? AND s.expires_at > NOW() LIMIT 1",
  )
  .bind(session_token)
  .fetch_optional(pool)
  .await
  .map_err(|error| format!("Falha ao validar sessão: {error}"))?;

  let Some(row) = row else {
    return Err("Sessão inválida ou expirada.".to_string());
  };

  let user = user_from_row(&row);
  if user.role != "admin" {
    return Err("Acesso restrito ao administrador.".to_string());
  }

  Ok(user)
}

async fn validate_session(pool: &MySqlPool, session_token: &str) -> Result<UserSummary, String> {
  let row = sqlx::query(
    "SELECT u.id, u.name, u.email, u.role, u.active FROM sessions s INNER JOIN users u ON u.id = s.user_id WHERE s.token = ? AND s.expires_at > NOW() LIMIT 1",
  )
  .bind(session_token)
  .fetch_optional(pool)
  .await
  .map_err(|error| format!("Falha ao validar sessão: {error}"))?;

  let Some(row) = row else {
    return Err("Sessão inválida ou expirada.".to_string());
  };

  Ok(user_from_row(&row))
}

async fn require_permission(pool: &MySqlPool, session_token: &str, module: &str, action: &str) -> Result<UserSummary, String> {
  let user = validate_session(pool, session_token).await?;
  if user.role == "admin" {
    return Ok(user);
  }

  let allowed: Option<bool> = match action {
    "view" => sqlx::query_scalar("SELECT can_view FROM user_permissions WHERE user_id = ? AND module_name = ? LIMIT 1"),
    "create" => sqlx::query_scalar("SELECT can_create FROM user_permissions WHERE user_id = ? AND module_name = ? LIMIT 1"),
    "edit" => sqlx::query_scalar("SELECT can_edit FROM user_permissions WHERE user_id = ? AND module_name = ? LIMIT 1"),
    "delete" => sqlx::query_scalar("SELECT can_delete FROM user_permissions WHERE user_id = ? AND module_name = ? LIMIT 1"),
    _ => return Err("Ação de permissão inválida.".to_string()),
  }
  .bind(user.id)
  .bind(module)
  .fetch_optional(pool)
  .await
  .map_err(|error| format!("Falha ao validar permissão: {error}"))?;

  if allowed.unwrap_or(false) {
    Ok(user)
  } else {
    Err("Você não possui permissão para esta operação.".to_string())
  }
}

async fn upsert_permissions_for_user(pool: &MySqlPool, user_id: i64, permissions: &[UserPermission]) -> Result<(), String> {
  for permission in permissions {
    sqlx::query(
      "INSERT INTO user_permissions (user_id, module_name, can_view, can_create, can_edit, can_delete) VALUES (?, ?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE can_view = VALUES(can_view), can_create = VALUES(can_create), can_edit = VALUES(can_edit), can_delete = VALUES(can_delete)",
    )
    .bind(user_id)
    .bind(&permission.module)
    .bind(permission.can_view)
    .bind(permission.can_create)
    .bind(permission.can_edit)
    .bind(permission.can_delete)
    .execute(pool)
    .await
    .map_err(|error| format!("Falha ao salvar permissões: {error}"))?;
  }

  Ok(())
}

fn product_from_row(row: &sqlx::mysql::MySqlRow) -> Product {
  Product {
    id: row.get("id"),
    code: row.get("code"),
    name: row.get("name"),
    manufacturer: row.get("manufacturer"),
    brand: row.get("brand"),
    supplier: row.get("supplier"),
    status: row.get("status"),
    street: row.get("street"),
    position: row.get("position"),
    level: row.get("level"),
    apartment: row.get("apartment"),
    quantity: row.get("quantity"),
  }
}

fn sale_from_row(row: &sqlx::mysql::MySqlRow) -> Sale {
  Sale {
    id: row.get("id"),
    client_name: row.get("client_name"),
    customer_type: row.get("customer_type"),
    item_count: row.get("item_count"),
    date: row.get("date"),
  }
}

fn validate_product_fields(payload: &ProductPayload) -> Result<(), String> {
  if payload.code.trim().is_empty() || payload.name.trim().is_empty() || payload.supplier.trim().is_empty() {
    return Err("Preencha código, nome e fornecedor para salvar o produto.".to_string());
  }
  if payload.quantity < 0 || [payload.street, payload.position, payload.level, payload.apartment].iter().any(|value| *value < 0 || *value > 99) {
    return Err("A quantidade não pode ser negativa.".to_string());
  }
  if payload.status != "ativo" && payload.status != "inativo" {
    return Err("Status de produto inválido.".to_string());
  }
  Ok(())
}

fn product_select() -> &'static str {
  "SELECT id, code, name, manufacturer, brand, supplier, status, street, position, level, apartment, quantity FROM products"
}

#[tauri::command]
async fn list_products(session_token: String, search: Option<String>, status: Option<String>) -> Result<Vec<Product>, String> {
  let pool = get_pool().await?;
  require_permission(&pool, &session_token, "estoque", "view").await?;
  let query = format!("{} WHERE (? IS NULL OR code LIKE CONCAT('%', ?, '%') OR name LIKE CONCAT('%', ?, '%') OR supplier LIKE CONCAT('%', ?, '%')) AND (? IS NULL OR status = ?) ORDER BY name ASC, id ASC", product_select());
  let term = search.as_deref();
  let rows = sqlx::query(&query)
    .bind(term).bind(term).bind(term).bind(term)
    .bind(status.as_deref()).bind(status.as_deref())
  .fetch_all(&pool)
  .await
  .map_err(|error| format!("Falha ao listar produtos: {error}"))?;

  Ok(rows.iter().map(product_from_row).collect())
}

#[tauri::command]
async fn create_product(payload: ProductPayload) -> Result<Product, String> {
  validate_product_fields(&payload)?;
  let pool = get_pool().await?;
  require_permission(&pool, &payload.session_token, "estoque", "create").await?;

  let result = sqlx::query(
    "INSERT INTO products (code, name, manufacturer, brand, supplier, status, street, position, level, apartment, quantity) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
  )
  .bind(payload.code.trim())
  .bind(payload.name.trim())
  .bind(payload.manufacturer.trim())
  .bind(payload.brand.trim())
  .bind(payload.supplier.trim())
  .bind(&payload.status)
  .bind(payload.street).bind(payload.position).bind(payload.level).bind(payload.apartment)
  .bind(payload.quantity)
  .execute(&pool)
  .await
  .map_err(|error| format!("Falha ao cadastrar produto: {error}"))?;

  let row = sqlx::query(
    &format!("{} WHERE id = ?", product_select()),
  )
  .bind(result.last_insert_id())
  .fetch_one(&pool)
  .await
  .map_err(|error| format!("Falha ao consultar produto cadastrado: {error}"))?;

  Ok(product_from_row(&row))
}

#[tauri::command]
async fn update_product(payload: UpdateProductPayload) -> Result<Product, String> {
  let create_payload = ProductPayload { code: payload.code.clone(), name: payload.name.clone(), manufacturer: payload.manufacturer.clone(), brand: payload.brand.clone(), supplier: payload.supplier.clone(), status: payload.status.clone(), street: payload.street, position: payload.position, level: payload.level, apartment: payload.apartment, quantity: payload.quantity, session_token: payload.session_token.clone() };
  validate_product_fields(&create_payload)?;
  let pool = get_pool().await?;
  require_permission(&pool, &payload.session_token, "estoque", "edit").await?;

  let result = sqlx::query(
    "UPDATE products SET code = ?, name = ?, manufacturer = ?, brand = ?, supplier = ?, status = ?, street = ?, position = ?, level = ?, apartment = ?, quantity = ? WHERE id = ?",
  )
  .bind(payload.code.trim())
  .bind(payload.name.trim())
  .bind(payload.manufacturer.trim())
  .bind(payload.brand.trim())
  .bind(payload.supplier.trim())
  .bind(&payload.status)
  .bind(payload.street).bind(payload.position).bind(payload.level).bind(payload.apartment)
  .bind(payload.quantity)
  .bind(payload.id)
  .execute(&pool)
  .await
  .map_err(|error| format!("Falha ao atualizar produto: {error}"))?;

  if result.rows_affected() == 0 {
    return Err("Produto não encontrado.".to_string());
  }

  let row = sqlx::query(
    &format!("{} WHERE id = ?", product_select()),
  )
  .bind(payload.id)
  .fetch_one(&pool)
  .await
  .map_err(|error| format!("Falha ao consultar produto atualizado: {error}"))?;

  Ok(product_from_row(&row))
}

#[tauri::command]
async fn delete_product(payload: DeleteProductPayload) -> Result<(), String> {
  let pool = get_pool().await?;
  require_permission(&pool, &payload.session_token, "estoque", "delete").await?;
  sqlx::query("DELETE FROM products WHERE id = ?")
    .bind(payload.id)
    .execute(&pool)
    .await
    .map_err(|error| format!("Não foi possível remover o produto. Verifique se ele possui vendas registradas: {error}"))?;
  Ok(())
}

#[tauri::command]
async fn list_sales(session_token: String) -> Result<Vec<Sale>, String> {
  let pool = get_pool().await?;
  let user = require_permission(&pool, &session_token, "relatorios", "view").await?;
  if user.role != "admin" && user.role != "pce" {
    return Err("Acesso restrito ao PCE ou administrador.".to_string());
  }
  let rows = sqlx::query(
    "SELECT s.id, s.client_name, s.customer_type, COUNT(si.id) AS item_count, DATE_FORMAT(s.created_at, '%d/%m/%Y %H:%i') AS date FROM sales s LEFT JOIN sale_items si ON si.sale_id = s.id GROUP BY s.id, s.client_name, s.customer_type, s.created_at ORDER BY s.created_at DESC, s.id DESC LIMIT 100",
  )
  .fetch_all(&pool)
  .await
  .map_err(|error| format!("Falha ao listar vendas: {error}"))?;

  Ok(rows.iter().map(sale_from_row).collect())
}

#[tauri::command]
async fn create_sale(payload: SalePayload) -> Result<Sale, String> {
  let client_name = payload.client_name.trim();
  if client_name.is_empty() {
    return Err("Informe o nome do cliente ou empresa.".to_string());
  }
  if payload.items.is_empty() || payload.items.iter().any(|item| item.quantity < 1) {
    return Err("Adicione pelo menos um produto com quantidade maior que zero.".to_string());
  }
  if payload.customer_type != "Pessoa física" && payload.customer_type != "Empresa" {
    return Err("Tipo de cliente inválido.".to_string());
  }

  let pool = get_pool().await?;
  require_permission(&pool, &payload.session_token, "vendas", "create").await?;
  let mut transaction = pool
    .begin()
    .await
    .map_err(|error| format!("Falha ao iniciar transação da venda: {error}"))?;

  let result = sqlx::query(
    "INSERT INTO sales (client_name, customer_type) VALUES (?, ?)",
  )
  .bind(client_name)
  .bind(&payload.customer_type)
  .execute(&mut *transaction)
  .await
  .map_err(|error| format!("Falha ao registrar venda: {error}"))?;

  let demand = sqlx::query("INSERT INTO picking_demands (sale_id) VALUES (?)")
    .bind(result.last_insert_id())
    .execute(&mut *transaction)
    .await
    .map_err(|error| format!("Falha ao criar demanda de separação: {error}"))?;

  for item in &payload.items {
    let product_row = sqlx::query(
      "SELECT id, code, name, quantity, street, position, level, apartment FROM products WHERE id = ? FOR UPDATE",
    )
    .bind(item.product_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|error| format!("Falha ao consultar estoque: {error}"))?;
    let Some(product_row) = product_row else {
      return Err("Um dos produtos selecionados não é válido.".to_string());
    };
    let stock: i64 = product_row.get("quantity");
    if item.quantity > stock {
      return Err(format!("Estoque insuficiente para o produto {}.", product_row.get::<String, _>("name")));
    }
    let product_name: String = product_row.get("name");
    let product_code: String = product_row.get("code");

    sqlx::query("UPDATE products SET quantity = quantity - ? WHERE id = ?")
      .bind(item.quantity).bind(item.product_id).execute(&mut *transaction).await
      .map_err(|error| format!("Falha ao atualizar estoque: {error}"))?;
    sqlx::query("INSERT INTO sale_items (sale_id, product_id, product_name, quantity) VALUES (?, ?, ?, ?)")
      .bind(result.last_insert_id()).bind(item.product_id).bind(&product_name).bind(item.quantity)
      .execute(&mut *transaction).await
      .map_err(|error| format!("Falha ao registrar item da venda: {error}"))?;
    sqlx::query("INSERT INTO picking_demand_items (demand_id, product_id, product_code, product_name, street, position, level, apartment, quantity) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
      .bind(demand.last_insert_id()).bind(item.product_id).bind(product_code).bind(&product_name)
      .bind(product_row.get::<i64, _>("street")).bind(product_row.get::<i64, _>("position"))
      .bind(product_row.get::<i64, _>("level")).bind(product_row.get::<i64, _>("apartment"))
      .bind(item.quantity).execute(&mut *transaction).await
      .map_err(|error| format!("Falha ao adicionar item à demanda: {error}"))?;
  }

  transaction
    .commit()
    .await
    .map_err(|error| format!("Falha ao confirmar venda: {error}"))?;

  let row = sqlx::query(
    "SELECT s.id, s.client_name, s.customer_type, COUNT(si.id) AS item_count, DATE_FORMAT(s.created_at, '%d/%m/%Y %H:%i') AS date FROM sales s LEFT JOIN sale_items si ON si.sale_id = s.id WHERE s.id = ? GROUP BY s.id, s.client_name, s.customer_type, s.created_at",
  )
  .bind(result.last_insert_id())
  .fetch_one(&pool)
  .await
  .map_err(|error| format!("Falha ao consultar venda registrada: {error}"))?;

  Ok(sale_from_row(&row))
}

#[tauri::command]
async fn list_demands(session_token: String, status: Option<String>) -> Result<Vec<PickingDemand>, String> {
  let pool = get_pool().await?;
  require_permission(&pool, &session_token, "demandas", "view").await?;
  let rows = sqlx::query("SELECT d.id, d.sale_id, d.status, DATE_FORMAT(d.created_at, '%d/%m/%Y %H:%i') AS created_at, s.client_name FROM picking_demands d INNER JOIN sales s ON s.id = d.sale_id WHERE (? IS NULL OR d.status = ?) ORDER BY d.created_at DESC, d.id DESC")
    .bind(status.as_deref()).bind(status.as_deref()).fetch_all(&pool).await
    .map_err(|error| format!("Falha ao listar demandas: {error}"))?;
  let mut demands = Vec::new();
  for row in rows {
    let demand_id: i64 = row.get("id");
    let items = sqlx::query("SELECT product_code, product_name, quantity, CONCAT(LPAD(street, 2, '0'), '-', LPAD(position, 2, '0'), '-', LPAD(level, 2, '0'), '-', LPAD(apartment, 2, '0')) AS location FROM picking_demand_items WHERE demand_id = ?")
      .bind(demand_id).fetch_all(&pool).await
      .map_err(|error| format!("Falha ao listar itens da demanda: {error}"))?
      .iter().map(|item| PickingDemandItem { code: item.get("product_code"), name: item.get("product_name"), quantity: item.get("quantity"), position: item.get("location") }).collect();
    demands.push(PickingDemand { id: demand_id, sale_id: row.get("sale_id"), status: row.get("status"), created_at: row.get("created_at"), client_name: row.get("client_name"), items });
  }
  Ok(demands)
}

#[tauri::command]
async fn complete_demand(payload: DemandStatusPayload) -> Result<(), String> {
  let pool = get_pool().await?;
  require_permission(&pool, &payload.session_token, "demandas", "edit").await?;
  sqlx::query("UPDATE picking_demands SET status = 'concluida', completed_at = NOW() WHERE id = ?")
    .bind(payload.demand_id).execute(&pool).await
    .map_err(|error| format!("Falha ao concluir demanda: {error}"))?;
  Ok(())
}

#[tauri::command]
async fn app_status() -> Result<AppStatus, String> {
  let pool = get_pool().await?;
  let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
    .fetch_one(&pool)
    .await
    .map_err(|error| format!("Falha ao consultar usuários: {error}"))?;

  Ok(AppStatus {
    needs_first_admin: count == 0,
    users_count: count,
  })
}

#[tauri::command]
async fn register_first_admin(payload: FirstAdminPayload) -> Result<AuthSession, String> {
  let name = payload.name.trim();
  let email = payload.email.trim();

  if name.is_empty() || email.is_empty() || payload.password.trim().is_empty() {
    return Err("Preencha nome, e-mail e senha para continuar.".to_string());
  }

  let pool = get_pool().await?;
  let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
    .fetch_one(&pool)
    .await
    .map_err(|error| format!("Falha ao verificar primeiro acesso: {error}"))?;

  if count > 0 {
    return Err("O primeiro administrador já foi criado.".to_string());
  }

  let existing: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE email = ? LIMIT 1")
    .bind(email)
    .fetch_optional(&pool)
    .await
    .map_err(|error| format!("Falha ao validar e-mail: {error}"))?;

  if existing.is_some() {
    return Err("Este e-mail já está cadastrado.".to_string());
  }

  let password_hash = hash_password(&payload.password)?;
  let result = sqlx::query(
    "INSERT INTO users (name, email, password_hash, role, active) VALUES (?, ?, ?, 'admin', TRUE)",
  )
  .bind(name)
  .bind(email)
  .bind(&password_hash)
  .execute(&pool)
  .await
  .map_err(|error| format!("Falha ao criar o administrador: {error}"))?;

  let user_id = result.last_insert_id() as i64;
  let token = session_token_for(user_id);
  let permissions = default_permissions_for_role("admin");

  upsert_permissions_for_user(&pool, user_id, &permissions).await?;

  sqlx::query(
    "INSERT INTO sessions (user_id, token, expires_at) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 30 DAY))",
  )
  .bind(user_id)
  .bind(&token)
  .execute(&pool)
  .await
  .map_err(|error| format!("Falha ao criar sessão: {error}"))?;

  let user = UserSummary {
    id: user_id,
    name: name.to_string(),
    email: email.to_string(),
    role: "admin".to_string(),
    active: true,
  };

  Ok(AuthSession {
    token,
    user,
    permissions,
  })
}

#[tauri::command]
async fn login(payload: LoginPayload) -> Result<AuthSession, String> {
  let email = payload.email.trim();
  if email.is_empty() || payload.password.trim().is_empty() {
    return Err("Informe e-mail e senha para entrar.".to_string());
  }

  let pool = get_pool().await?;
  let row = sqlx::query(
    "SELECT id, name, email, password_hash, role, active FROM users WHERE email = ? AND active = TRUE LIMIT 1",
  )
  .bind(email)
  .fetch_optional(&pool)
  .await
  .map_err(|error| format!("Falha ao buscar usuário: {error}"))?;

  let Some(row) = row else {
    return Err("Credenciais inválidas.".to_string());
  };

  let user = user_from_row(&row);
  let stored_hash: String = row.get("password_hash");
  let is_valid = verify_password(&payload.password, &stored_hash)?;

  if !is_valid {
    return Err("Credenciais inválidas.".to_string());
  }

  let token = session_token_for(user.id);
  let permissions = list_permissions_for_user(&pool, user.id).await?;

  sqlx::query(
    "INSERT INTO sessions (user_id, token, expires_at) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 30 DAY))",
  )
  .bind(user.id)
  .bind(&token)
  .execute(&pool)
  .await
  .map_err(|error| format!("Falha ao criar sessão do login: {error}"))?;

  Ok(AuthSession {
    token,
    user,
    permissions,
  })
}

#[tauri::command]
async fn list_users(session_token: String) -> Result<Vec<UserSummary>, String> {
  let pool = get_pool().await?;
  require_admin_session(&pool, &session_token).await?;

  let rows = sqlx::query("SELECT id, name, email, role, active FROM users ORDER BY created_at DESC")
    .fetch_all(&pool)
    .await
    .map_err(|error| format!("Falha ao listar usuários: {error}"))?;

  let mut users = Vec::new();
  for row in rows {
    users.push(user_from_row(&row));
  }

  Ok(users)
}

#[tauri::command]
async fn create_user(payload: CreateUserPayload) -> Result<UserSummary, String> {
  let name = payload.name.trim();
  let email = payload.email.trim();
  let password = payload.password.trim();

  if name.is_empty() || email.is_empty() || password.is_empty() {
    return Err("Preencha todos os campos do usuário.".to_string());
  }

  let pool = get_pool().await?;
  require_admin_session(&pool, &payload.session_token).await?;

  let role = match payload.role.as_str() {
    "admin" | "vendedor" | "estoquista" | "pce" => payload.role,
    _ => return Err("Perfil de usuário inválido.".to_string()),
  };

  let existing: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE email = ? LIMIT 1")
    .bind(email)
    .fetch_optional(&pool)
    .await
    .map_err(|error| format!("Falha ao verificar e-mail: {error}"))?;

  if existing.is_some() {
    return Err("Já existe um usuário com este e-mail.".to_string());
  }

  let password_hash = hash_password(password)?;
  let result = sqlx::query(
    "INSERT INTO users (name, email, password_hash, role, active) VALUES (?, ?, ?, ?, TRUE)",
  )
  .bind(name)
  .bind(email)
  .bind(&password_hash)
  .bind(&role)
  .execute(&pool)
  .await
  .map_err(|error| format!("Falha ao criar usuário: {error}"))?;

  let user_id = result.last_insert_id() as i64;
  let permissions = payload.permissions.unwrap_or_else(|| default_permissions_for_role(&role));
  upsert_permissions_for_user(&pool, user_id, &permissions).await?;

  Ok(UserSummary {
    id: user_id,
    name: name.to_string(),
    email: email.to_string(),
    role,
    active: true,
  })
}

#[tauri::command]
async fn list_user_permissions(session_token: String, user_id: Option<i64>) -> Result<Vec<UserPermission>, String> {
  let pool = get_pool().await?;
  let current_user = validate_session(&pool, &session_token).await?;

  let target_user_id = match user_id {
    Some(id) => {
      if current_user.role != "admin" {
        return Err("Acesso restrito ao administrador.".to_string());
      }
      id
    }
    None => current_user.id,
  };

  list_permissions_for_user(&pool, target_user_id).await
}

#[tauri::command]
async fn update_user_permissions(payload: UpdatePermissionsPayload) -> Result<Vec<UserPermission>, String> {
  let pool = get_pool().await?;
  require_admin_session(&pool, &payload.session_token).await?;
  upsert_permissions_for_user(&pool, payload.user_id, &payload.permissions).await?;
  list_permissions_for_user(&pool, payload.user_id).await
}

#[tauri::command]
async fn logout(session_token: String) -> Result<(), String> {
  let pool = get_pool().await?;
  sqlx::query("DELETE FROM sessions WHERE token = ?")
    .bind(session_token)
    .execute(&pool)
    .await
    .map_err(|error| format!("Falha ao encerrar sessão: {error}"))?;

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      app_status,
      register_first_admin,
      login,
      list_users,
      create_user,
      list_user_permissions,
      update_user_permissions,
      logout,
      list_products,
      create_product,
      update_product,
      delete_product,
      list_sales,
      create_sale,
      list_demands,
      complete_demand
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
