<script setup>
import { computed, ref } from 'vue'

function loadState(key, fallback) {
  if (typeof window === 'undefined') {
    return fallback
  }

  try {
    const saved = window.localStorage.getItem(key)
    return saved ? JSON.parse(saved) : fallback
  } catch {
    return fallback
  }
}

function persistState() {
  if (typeof window !== 'undefined') {
    window.localStorage.setItem('erp-products', JSON.stringify(products.value))
    window.localStorage.setItem('erp-sales', JSON.stringify(sales.value))
  }
}

const products = ref(loadState('erp-products', []))
const sales = ref(loadState('erp-sales', []))
const productForm = ref({
  name: '',
  manufacturer: '',
  brand: '',
  quantity: 1,
})
const saleForm = ref({
  productId: '',
  clientName: '',
  customerType: 'Pessoa física',
  quantity: 1,
})
const editingProductId = ref(null)
const feedback = ref({ type: '', message: '' })
const fileInput = ref(null)

function setFeedback(type, message) {
  feedback.value = { type, message }
}

function downloadDatabase() {
  if (typeof window === 'undefined') {
    return
  }

  const payload = {
    exportedAt: new Date().toISOString(),
    products: products.value,
    sales: sales.value,
  }

  const blob = new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' })
  const url = window.URL.createObjectURL(blob)
  const link = document.createElement('a')

  link.href = url
  link.download = `inventory-db-${new Date().toISOString().slice(0, 10)}.json`
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  window.URL.revokeObjectURL(url)

  setFeedback('success', 'Banco exportado com sucesso.')
}

function triggerImport() {
  fileInput.value?.click()
}

function handleImport(event) {
  const file = event.target.files?.[0]
  if (!file) {
    return
  }

  const reader = new FileReader()

  reader.onload = () => {
    try {
      if (typeof reader.result !== 'string') {
        throw new Error('Arquivo inválido')
      }

      const parsed = JSON.parse(reader.result)
      const importedProducts = Array.isArray(parsed.products) ? parsed.products : []
      const importedSales = Array.isArray(parsed.sales) ? parsed.sales : []

      products.value = importedProducts
      sales.value = importedSales
      persistState()
      resetSaleForm()
      setFeedback('success', 'Dados importados com sucesso. Estoque e vendas foram atualizados.')
    } catch {
      setFeedback('error', 'Não foi possível importar o arquivo JSON.')
    } finally {
      event.target.value = ''
    }
  }

  reader.onerror = () => {
    setFeedback('error', 'Não foi possível ler o arquivo selecionado.')
    event.target.value = ''
  }

  reader.readAsText(file)
}

function resetProductForm() {
  productForm.value = { name: '', manufacturer: '', brand: '', quantity: 1 }
  editingProductId.value = null
}

function resetSaleForm() {
  saleForm.value = {
    productId: products.value[0]?.id || '',
    clientName: '',
    customerType: 'Pessoa física',
    quantity: 1,
  }
}

function handleProductSubmit() {
  if (!productForm.value.name.trim() || !productForm.value.manufacturer.trim() || !productForm.value.brand.trim()) {
    setFeedback('error', 'Preencha nome, fabricante e marca para salvar o produto.')
    return
  }

  const quantity = Number(productForm.value.quantity)
  if (!Number.isInteger(quantity) || quantity < 1) {
    setFeedback('error', 'A quantidade deve ser um número inteiro maior que zero.')
    return
  }

  if (editingProductId.value) {
    products.value = products.value.map((product) =>
      product.id === editingProductId.value
        ? { ...product, ...productForm.value, quantity }
        : product,
    )
    setFeedback('success', 'Produto atualizado com sucesso.')
  } else {
    products.value = [
      ...products.value,
      {
        id: Date.now(),
        name: productForm.value.name.trim(),
        manufacturer: productForm.value.manufacturer.trim(),
        brand: productForm.value.brand.trim(),
        quantity,
      },
    ]
    setFeedback('success', 'Produto cadastrado com sucesso.')
  }

  persistState()
  resetProductForm()
  resetSaleForm()
}

function editProduct(product) {
  productForm.value = {
    name: product.name,
    manufacturer: product.manufacturer,
    brand: product.brand,
    quantity: product.quantity,
  }
  editingProductId.value = product.id
  setFeedback('info', 'Você está editando um produto existente.')
}

function removeProduct(productId) {
  products.value = products.value.filter((product) => product.id !== productId)
  if (saleForm.value.productId === productId) {
    resetSaleForm()
  }
  persistState()
  setFeedback('success', 'Produto removido.')
}

function handleSaleSubmit() {
  const product = products.value.find((item) => item.id === saleForm.value.productId)
  if (!product) {
    setFeedback('error', 'Selecione um produto válido para vender.')
    return
  }

  const quantity = Number(saleForm.value.quantity)
  if (!saleForm.value.clientName.trim()) {
    setFeedback('error', 'Informe o nome do cliente ou empresa.')
    return
  }
  if (!Number.isInteger(quantity) || quantity < 1) {
    setFeedback('error', 'A quantidade da venda deve ser um número inteiro maior que zero.')
    return
  }
  if (quantity > product.quantity) {
    setFeedback('error', 'Estoque insuficiente para esta venda.')
    return
  }

  products.value = products.value.map((item) =>
    item.id === product.id ? { ...item, quantity: item.quantity - quantity } : item,
  )

  sales.value = [
    {
      id: Date.now(),
      productName: product.name,
      clientName: saleForm.value.clientName.trim(),
      customerType: saleForm.value.customerType,
      quantity,
      date: new Date().toLocaleString('pt-BR'),
    },
    ...sales.value,
  ]

  persistState()
  setFeedback('success', 'Venda registrada e estoque atualizado.')
  resetSaleForm()
}

const totalProducts = computed(() => products.value.length)
const totalStock = computed(() => products.value.reduce((sum, product) => sum + product.quantity, 0))
const lowStockProducts = computed(() => products.value.filter((product) => product.quantity <= 5))
</script>

<template>
  <div class="app-shell">
    <header class="hero-card">
      <div>
        <p class="eyebrow">ERP básico</p>
        <h1>Controle de estoque e vendas</h1>
        <p class="subtitle">
          Cadastre produtos, ajuste informações e registre vendas com baixa automática no estoque.
        </p>
      </div>
      <div class="stats-grid">
        <div class="stat-card">
          <strong>{{ totalProducts }}</strong>
          <span>Produtos</span>
        </div>
        <div class="stat-card">
          <strong>{{ totalStock }}</strong>
          <span>Unidades em estoque</span>
        </div>
        <div class="stat-card">
          <strong>{{ sales.length }}</strong>
          <span>Vendas registradas</span>
        </div>
      </div>
    </header>

    <div v-if="feedback.message" :class="['feedback', feedback.type]">
      {{ feedback.message }}
    </div>

    <section class="grid">
      <div class="panel">
        <div class="panel-header">
          <h2>{{ editingProductId ? 'Editar produto' : 'Adicionar produto' }}</h2>
          <button v-if="editingProductId" class="ghost-btn" @click="resetProductForm">
            Cancelar
          </button>
        </div>

        <form class="form-stack" @submit.prevent="handleProductSubmit">
          <label>
            Nome do produto
            <input v-model="productForm.name" placeholder="Ex.: Teclado mecânico" />
          </label>
          <label>
            Fabricante
            <input v-model="productForm.manufacturer" placeholder="Ex.: TechLabs" />
          </label>
          <label>
            Marca
            <input v-model="productForm.brand" placeholder="Ex.: HyperKey" />
          </label>
          <label>
            Quantidade
            <input v-model.number="productForm.quantity" type="number" min="1" />
          </label>
          <button type="submit" class="primary-btn">
            {{ editingProductId ? 'Salvar alterações' : 'Cadastrar produto' }}
          </button>
        </form>
      </div>

      <div class="panel">
        <div class="panel-header">
          <h2>Produtos cadastrados</h2>
          <span class="badge">{{ products.length }} itens</span>
        </div>

        <div v-if="products.length" class="product-list">
          <article v-for="product in products" :key="product.id" class="product-card">
            <div>
              <h3>{{ product.name }}</h3>
              <p>{{ product.manufacturer }} · {{ product.brand }}</p>
              <p class="stock">Estoque: {{ product.quantity }}</p>
            </div>
            <div class="card-actions">
              <button class="ghost-btn" @click="editProduct(product)">Editar</button>
              <button class="danger-btn" @click="removeProduct(product.id)">Remover</button>
            </div>
          </article>
        </div>
        <p v-else class="empty-state">Nenhum produto cadastrado ainda.</p>
      </div>
    </section>

    <section class="grid">
      <div class="panel">
        <div class="panel-header">
          <h2>Registrar venda</h2>
          <span class="badge">Baixa no estoque</span>
        </div>

        <form class="form-stack" @submit.prevent="handleSaleSubmit">
          <label>
            Produto
            <select v-model="saleForm.productId">
              <option value="" disabled>Selecione um produto</option>
              <option v-for="product in products" :key="product.id" :value="product.id">
                {{ product.name }} (estoque: {{ product.quantity }})
              </option>
            </select>
          </label>
          <label>
            Cliente / Empresa
            <input v-model="saleForm.clientName" placeholder="Ex.: João da Silva" />
          </label>
          <label>
            Tipo de cliente
            <select v-model="saleForm.customerType">
              <option value="Pessoa física">Pessoa física</option>
              <option value="Empresa">Empresa</option>
            </select>
          </label>
          <label>
            Quantidade da venda
            <input v-model.number="saleForm.quantity" type="number" min="1" />
          </label>
          <button type="submit" class="primary-btn">Confirmar venda</button>
        </form>
      </div>

      <div class="panel">
        <div class="panel-header">
          <h2>Resumo</h2>
          <span class="badge danger">{{ lowStockProducts.length }} com estoque baixo</span>
        </div>

        <div v-if="lowStockProducts.length" class="warning-list">
          <p v-for="product in lowStockProducts" :key="product.id">
            {{ product.name }} · {{ product.quantity }} unidade(s)
          </p>
        </div>
        <p v-else class="empty-state">Estoque saudável para todos os produtos.</p>

        <div class="sales-list">
          <h3>Últimas vendas</h3>
          <div v-if="sales.length" class="sale-card-list">
            <article v-for="sale in sales.slice(0, 5)" :key="sale.id" class="sale-card">
              <strong>{{ sale.productName }}</strong>
              <p>{{ sale.clientName }} · {{ sale.customerType }}</p>
              <p>{{ sale.quantity }} unidade(s) · {{ sale.date }}</p>
            </article>
          </div>
          <p v-else class="empty-state">Nenhuma venda registrada até o momento.</p>
        </div>
      </div>
    </section>

    <div class="db-actions">
      <button class="ghost-btn" type="button" @click="downloadDatabase">Download DB</button>
      <button class="primary-btn" type="button" @click="triggerImport">import DB</button>
      <input ref="fileInput" type="file" accept="application/json" hidden @change="handleImport" />
    </div>
  </div>
</template>

<style scoped>
:global(body) {
  margin: 0;
  font-family: Inter, 'Segoe UI', sans-serif;
  background: linear-gradient(135deg, #f3f6ff 0%, #eef4ff 100%);
  color: #14213d;
}

* {
  box-sizing: border-box;
}

button,
input,
select {
  font: inherit;
}

.app-shell {
  max-width: 1200px;
  margin: 0 auto;
  padding: 24px;
}

.hero-card {
  background: linear-gradient(135deg, #0f172a 0%, #1d4ed8 100%);
  color: white;
  border-radius: 24px;
  padding: 24px;
  display: flex;
  justify-content: space-between;
  gap: 20px;
  align-items: center;
  box-shadow: 0 18px 45px rgba(15, 23, 42, 0.18);
}

.eyebrow {
  text-transform: uppercase;
  letter-spacing: 0.3em;
  font-size: 0.75rem;
  opacity: 0.8;
  margin: 0 0 6px;
}

.hero-card h1 {
  margin: 0 0 8px;
  font-size: 1.85rem;
}

.subtitle {
  margin: 0;
  max-width: 560px;
  opacity: 0.92;
}

.stats-grid {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.stat-card {
  min-width: 110px;
  padding: 12px 14px;
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.16);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-card strong {
  font-size: 1.25rem;
}

.feedback {
  margin: 16px 0;
  padding: 12px 14px;
  border-radius: 12px;
  font-weight: 600;
}

.feedback.success {
  background: #dcfce7;
  color: #166534;
}

.feedback.error {
  background: #fee2e2;
  color: #b91c1c;
}

.feedback.info {
  background: #e0f2fe;
  color: #075985;
}

.grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.panel {
  background: white;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.08);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 14px;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.1rem;
}

.badge {
  font-size: 0.8rem;
  padding: 6px 10px;
  border-radius: 999px;
  background: #e0e7ff;
  color: #3730a3;
}

.badge.danger {
  background: #fee2e2;
  color: #b91c1c;
}

.form-stack {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

label {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-weight: 600;
  font-size: 0.95rem;
}

input,
select {
  border: 1px solid #dbe3f0;
  border-radius: 10px;
  padding: 10px 12px;
  background: #f8fafc;
}

.primary-btn,
.ghost-btn,
.danger-btn {
  border: none;
  border-radius: 10px;
  padding: 10px 12px;
  cursor: pointer;
}

.primary-btn {
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  font-weight: 700;
}

.ghost-btn {
  background: #e2e8f0;
  color: #0f172a;
}

.danger-btn {
  background: #fee2e2;
  color: #b91c1c;
}

.product-list,
.sale-card-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.product-card,
.sale-card {
  border: 1px solid #e5e7eb;
  border-radius: 14px;
  padding: 12px;
  display: flex;
  justify-content: space-between;
  gap: 10px;
  align-items: center;
}

.product-card h3,
.sale-card strong {
  margin: 0 0 4px;
}

.product-card p,
.sale-card p {
  margin: 2px 0;
  color: #475569;
}

.stock {
  font-weight: 700;
  color: #0f766e;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.empty-state {
  color: #64748b;
  margin: 8px 0 0;
}

.db-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  margin-top: 24px;
}

.db-actions button {
  min-width: 140px;
}

.warning-list {
  padding: 10px 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

@media (max-width: 900px) {
  .grid {
    grid-template-columns: 1fr;
  }

  .hero-card {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
