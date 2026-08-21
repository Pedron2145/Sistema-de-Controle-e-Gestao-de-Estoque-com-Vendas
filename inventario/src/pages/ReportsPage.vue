<script setup>
import { computed, onMounted } from 'vue'
import { useInventory } from '../composables/useInventory'
import { useAuth } from '../composables/useAuth'

const { products, sales, totalProducts, totalStock } = useInventory()
const { loadData } = useInventory()
const { hasPermission } = useAuth()

const recentSales = computed(() => sales.value.slice(0, 5))
const lowStockProducts = computed(() => products.value.filter((product) => product.quantity <= 5))
const canViewReports = hasPermission('relatorios', 'can_view')

onMounted(() => {
  loadData()
})
</script>

<template>
  <div class="page-shell">
    <template v-if="canViewReports">
      <header class="hero-card">
        <div>
          <p class="eyebrow">Relatórios</p>
          <h1>Dashboard operacional</h1>
          <p class="subtitle">
            Acompanhamento rápido do volume de produtos, vendas e alertas de estoque.
          </p>
        </div>

        <div class="stats-grid">
          <div class="stat-card">
            <strong>{{ totalProducts }}</strong>
            <span>Produtos</span>
          </div>
          <div class="stat-card">
            <strong>{{ totalStock }}</strong>
            <span>Unidades</span>
          </div>
          <div class="stat-card">
            <strong>{{ sales.length }}</strong>
            <span>Vendas</span>
          </div>
        </div>
      </header>

      <section class="panel-grid">
        <div class="panel">
          <div class="panel-header">
            <h2>Últimas vendas</h2>
            <span class="badge">Resumo</span>
          </div>

          <div v-if="recentSales.length" class="table-wrap">
            <table>
              <thead>
                <tr>
                  <th>Cliente</th>
                  <th>Produto</th>
                  <th>Qtd.</th>
                  <th>Data</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="sale in recentSales" :key="sale.id">
                  <td>{{ sale.clientName }}</td>
                  <td>{{ sale.productName }}</td>
                  <td>{{ sale.quantity }}</td>
                  <td>{{ sale.date }}</td>
                </tr>
              </tbody>
            </table>
          </div>
          <p v-else class="empty-state">Ainda não há vendas registradas.</p>
        </div>

        <div class="panel">
          <div class="panel-header">
            <h2>Alerta de estoque</h2>
            <span class="badge danger">Crítico</span>
          </div>

          <div v-if="lowStockProducts.length" class="list-stack">
            <div v-for="product in lowStockProducts" :key="product.id" class="alert-item">
              <strong>{{ product.name }}</strong>
              <span>{{ product.quantity }} unidades restantes</span>
            </div>
          </div>
          <p v-else class="empty-state">Todos os produtos estão com estoque estável.</p>
        </div>
      </section>
    </template>

    <div v-else class="panel lock-panel">
      <h2>Relatórios bloqueados</h2>
      <p>Seu perfil não possui acesso a este módulo.</p>
    </div>
  </div>
</template>

<style scoped>
.page-shell {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.hero-card {
  display: flex;
  justify-content: space-between;
  gap: 20px;
  padding: 24px;
  border-radius: 24px;
  background: linear-gradient(135deg, #0f172a 0%, #1d4ed8 100%);
  color: white;
  box-shadow: 0 18px 45px rgba(15, 23, 42, 0.18);
}

.eyebrow {
  margin: 0 0 8px;
  font-size: 0.75rem;
  letter-spacing: 0.3em;
  text-transform: uppercase;
  opacity: 0.8;
}

h1 {
  margin: 0 0 8px;
  font-size: 1.8rem;
}

.subtitle {
  max-width: 520px;
  margin: 0;
  opacity: 0.9;
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

.panel-grid {
  display: grid;
  grid-template-columns: 1.5fr 1fr;
  gap: 20px;
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

.table-wrap {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  text-align: left;
  padding: 10px 8px;
  border-bottom: 1px solid #e2e8f0;
}

.list-stack {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.alert-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  background: #f8fafc;
  border: 1px solid #dbe3f0;
  border-radius: 12px;
  padding: 12px 14px;
}

.empty-state {
  margin: 0;
  color: #475569;
}

.lock-panel {
  background: white;
  border-radius: 20px;
  padding: 24px;
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.08);
}

.lock-panel h2 {
  margin: 0 0 8px;
}

.lock-panel p {
  margin: 0;
  color: #475569;
}
</style>

