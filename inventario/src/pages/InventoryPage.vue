<script setup>
import { onMounted, ref } from 'vue'
import HeaderStats from '../components/HeaderStats.vue'
import Feedback from '../components/Feedback.vue'
import ProductForm from '../components/ProductForm.vue'
import ProductList from '../components/ProductList.vue'
import SummaryPanel from '../components/SummaryPanel.vue'
import { useInventory } from '../composables/useInventory'
import { useAuth } from '../composables/useAuth'

const { loadData, searchProducts } = useInventory()
const { hasPermission } = useAuth()

const canCreateStock = hasPermission('estoque', 'can_create')
const canEditStock = hasPermission('estoque', 'can_edit')
const canDeleteStock = hasPermission('estoque', 'can_delete')
const search = ref('')
const status = ref('')

onMounted(() => {
  loadData()
})

async function applySearch() {
  await searchProducts(search.value, status.value || null)
}
</script>

<template>
  <div class="page-shell">
    <HeaderStats />
    <Feedback />

    <section class="panel search-panel">
      <div class="panel-header">
        <div>
          <h2>Pesquisar produto</h2>
          <p class="panel-hint">Consulte código, fornecedor, status e posição Rua-Posição-Nível-Apartamento.</p>
        </div>
      </div>
      <form class="search-row" @submit.prevent="applySearch">
        <input v-model="search" placeholder="Nome, código ou fornecedor" />
        <select v-model="status">
          <option value="">Todos os status</option>
          <option value="ativo">Ativos</option>
          <option value="inativo">Inativos</option>
        </select>
        <button class="primary-btn" type="submit">Pesquisar</button>
      </form>
    </section>

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

    <SummaryPanel />

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

.search-row {
  display: grid;
  grid-template-columns: 1fr 180px auto;
  gap: 10px;
}

.search-row input,
.search-row select {
  border: 1px solid #dbe3f0;
  border-radius: 11px;
  background: #f8fafc;
  padding: 10px 12px;
}

.panel-hint {
  margin: 6px 0 0;
  color: #64748b;
  font-size: 0.9rem;
}

@media (max-width: 760px) {
  .search-row {
    grid-template-columns: 1fr;
  }
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
