import { ref, computed } from 'vue'

function loadState(key, fallback) {
  if (typeof window === 'undefined') return fallback
  try {
    const saved = window.localStorage.getItem(key)
    return saved ? JSON.parse(saved) : fallback
  } catch {
    return fallback
  }
}

function persistState(products, sales) {
  if (typeof window === 'undefined') return
  window.localStorage.setItem('erp-products', JSON.stringify(products.value))
  window.localStorage.setItem('erp-sales', JSON.stringify(sales.value))
}

const products = ref(loadState('erp-products', []))
const sales = ref(loadState('erp-sales', []))
const productForm = ref({ name: '', manufacturer: '', brand: '', quantity: 1 })
const saleForm = ref({ productId: '', clientName: '', customerType: 'Pessoa física', quantity: 1 })
const editingProductId = ref(null)
const feedback = ref({ type: '', message: '' })
const fileInput = ref(null)

function setFeedback(type, message) {
  feedback.value = { type, message }
}

function downloadDatabase() {
  if (typeof window === 'undefined') return
  const payload = { exportedAt: new Date().toISOString(), products: products.value, sales: sales.value }
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
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    try {
      if (typeof reader.result !== 'string') throw new Error('Arquivo inválido')
      const parsed = JSON.parse(reader.result)
      const importedProducts = Array.isArray(parsed.products) ? parsed.products : []
      const importedSales = Array.isArray(parsed.sales) ? parsed.sales : []
      products.value = importedProducts
      sales.value = importedSales
      persistState(products, sales)
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
  saleForm.value = { productId: products.value[0]?.id || '', clientName: '', customerType: 'Pessoa física', quantity: 1 }
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
      product.id === editingProductId.value ? { ...product, ...productForm.value, quantity } : product,
    )
    setFeedback('success', 'Produto atualizado com sucesso.')
  } else {
    products.value = [
      ...products.value,
      { id: Date.now(), name: productForm.value.name.trim(), manufacturer: productForm.value.manufacturer.trim(), brand: productForm.value.brand.trim(), quantity },
    ]
    setFeedback('success', 'Produto cadastrado com sucesso.')
  }
  persistState(products, sales)
  resetProductForm()
  resetSaleForm()
}

function editProduct(product) {
  productForm.value = { name: product.name, manufacturer: product.manufacturer, brand: product.brand, quantity: product.quantity }
  editingProductId.value = product.id
  setFeedback('info', 'Você está editando um produto existente.')
}

function removeProduct(productId) {
  products.value = products.value.filter((product) => product.id !== productId)
  if (saleForm.value.productId === productId) resetSaleForm()
  persistState(products, sales)
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
  products.value = products.value.map((item) => (item.id === product.id ? { ...item, quantity: item.quantity - quantity } : item))
  sales.value = [
    { id: Date.now(), productName: product.name, clientName: saleForm.value.clientName.trim(), customerType: saleForm.value.customerType, quantity, date: new Date().toLocaleString('pt-BR') },
    ...sales.value,
  ]
  persistState(products, sales)
  setFeedback('success', 'Venda registrada e estoque atualizado.')
  resetSaleForm()
}

const totalProducts = computed(() => products.value.length)
const totalStock = computed(() => products.value.reduce((sum, product) => sum + product.quantity, 0))
const lowStockProducts = computed(() => products.value.filter((product) => product.quantity <= 5))

export function useInventory() {
  return {
    products,
    sales,
    productForm,
    saleForm,
    editingProductId,
    feedback,
    fileInput,
    setFeedback,
    downloadDatabase,
    triggerImport,
    handleImport,
    resetProductForm,
    resetSaleForm,
    handleProductSubmit,
    editProduct,
    removeProduct,
    handleSaleSubmit,
    totalProducts,
    totalStock,
    lowStockProducts,
  }
}

export default useInventory
