<script setup>
import { onMounted } from 'vue'
import HeaderStats from '../components/HeaderStats.vue'
import Feedback from '../components/Feedback.vue'
import ProductForm from '../components/ProductForm.vue'
import ProductList from '../components/ProductList.vue'
import SaleForm from '../components/SaleForm.vue'
import SummaryPanel from '../components/SummaryPanel.vue'
import { useInventory } from '../composables/useInventory'
import { useAuth } from '../composables/useAuth'

const { resetSaleForm, loadData } = useInventory()
const { hasPermission } = useAuth()

const canCreateStock = hasPermission('estoque', 'can_create')
const canEditStock = hasPermission('estoque', 'can_edit')
const canDeleteStock = hasPermission('estoque', 'can_delete')

onMounted(() => {
  loadData()
  resetSaleForm()
})
</script>

<template>
  <div class="page-shell">
    <HeaderStats />
    <Feedback />

    <section class="grid">
      <div v-if="canCreateStock || canEditStock || canDeleteStock" class="stock-form-wrap">
        <ProductForm />
      </div>
      <div v-else class="panel readonly-panel">
        <h2>Estoque em modo somente leitura</h2>
        <p>Seu perfil permite visualizar o estoque, mas não alterar ou cadastrar itens.</p>
      </div>
      <ProductList />
    </section>

    <section class="grid">
      <SaleForm />
      <SummaryPanel />
    </section>

  </div>
</template>

<style scoped>
.page-shell {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 20px;
}

.stock-form-wrap {
  min-width: 0;
}

.readonly-panel {
  background: white;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.08);
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.readonly-panel h2 {
  margin: 0 0 8px;
}

.readonly-panel p {
  margin: 0;
  color: #475569;
}
</style>
