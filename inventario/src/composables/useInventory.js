import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAuth } from './useAuth'

const products = ref([])
const sales = ref([])
const productForm = ref({ name: '', manufacturer: '', brand: '', quantity: 1 })
const saleForm = ref({ productId: '', clientName: '', customerType: 'Pessoa física', quantity: 1 })
const editingProductId = ref(null)
const feedback = ref({ type: '', message: '' })
const loading = ref(false)

const { session, hasPermission } = useAuth()

function setFeedback(type, message) {
  feedback.value = { type, message }
}

function sessionToken() {
  if (!session.value?.token) {
    throw new Error('Sessão não encontrada. Faça login novamente.')
  }
  return session.value.token
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

async function loadData() {
  if (!session.value?.token) return
  loading.value = true
  try {
    const loadedProducts = await invoke('list_products', { session_token: sessionToken() })
    const loadedSales = hasPermission('relatorios', 'can_view')
      ? await invoke('list_sales', { session_token: sessionToken() })
      : []
    products.value = loadedProducts
    sales.value = loadedSales
    resetSaleForm()
  } catch (error) {
    setFeedback('error', error?.toString() ?? 'Não foi possível carregar os dados do MySQL.')
  } finally {
    loading.value = false
  }
}

async function handleProductSubmit() {
  const name = productForm.value.name.trim()
  const manufacturer = productForm.value.manufacturer.trim()
  const brand = productForm.value.brand.trim()
  const quantity = Number(productForm.value.quantity)

  if (!name || !manufacturer || !brand) {
    setFeedback('error', 'Preencha nome, fabricante e marca para salvar o produto.')
    return
  }
  if (!Number.isInteger(quantity) || quantity < 0) {
    setFeedback('error', 'A quantidade deve ser um número inteiro maior ou igual a zero.')
    return
  }

  loading.value = true
  try {
    const payload = { name, manufacturer, brand, quantity, session_token: sessionToken() }
    const savedProduct = editingProductId.value
      ? await invoke('update_product', { payload: { ...payload, id: editingProductId.value } })
      : await invoke('create_product', { payload })

    if (editingProductId.value) {
      products.value = products.value.map((product) => (product.id === savedProduct.id ? savedProduct : product))
      setFeedback('success', 'Produto atualizado com sucesso.')
    } else {
      products.value = [...products.value, savedProduct].sort((first, second) => first.name.localeCompare(second.name))
      setFeedback('success', 'Produto cadastrado com sucesso.')
    }
    resetProductForm()
    resetSaleForm()
  } catch (error) {
    setFeedback('error', error?.toString() ?? 'Não foi possível salvar o produto.')
  } finally {
    loading.value = false
  }
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

async function removeProduct(productId) {
  loading.value = true
  try {
    await invoke('delete_product', {
      payload: { id: productId, session_token: sessionToken() },
    })
    products.value = products.value.filter((product) => product.id !== productId)
    resetSaleForm()
    setFeedback('success', 'Produto removido.')
  } catch (error) {
    setFeedback('error', error?.toString() ?? 'Não foi possível remover o produto.')
  } finally {
    loading.value = false
  }
}

async function handleSaleSubmit() {
  const quantity = Number(saleForm.value.quantity)
  if (!saleForm.value.productId) {
    setFeedback('error', 'Selecione um produto válido para vender.')
    return
  }
  if (!saleForm.value.clientName.trim()) {
    setFeedback('error', 'Informe o nome do cliente ou empresa.')
    return
  }
  if (!Number.isInteger(quantity) || quantity < 1) {
    setFeedback('error', 'A quantidade da venda deve ser um número inteiro maior que zero.')
    return
  }

  loading.value = true
  try {
    const sale = await invoke('create_sale', {
      payload: {
        product_id: saleForm.value.productId,
        client_name: saleForm.value.clientName.trim(),
        customer_type: saleForm.value.customerType,
        quantity,
        session_token: sessionToken(),
      },
    })
    await loadData()
    sales.value = [sale, ...sales.value.filter((item) => item.id !== sale.id)]
    setFeedback('success', 'Venda registrada e estoque atualizado.')
    resetSaleForm()
  } catch (error) {
    setFeedback('error', error?.toString() ?? 'Não foi possível registrar a venda.')
  } finally {
    loading.value = false
  }
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
    loading,
    setFeedback,
    loadData,
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
