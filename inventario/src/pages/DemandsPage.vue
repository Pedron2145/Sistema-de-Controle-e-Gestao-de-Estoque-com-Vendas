<script setup>
import { computed, onMounted, ref } from 'vue'
import { useInventory } from '../composables/useInventory'

const { demands, loadDemands, completeDemand } = useInventory()
const filter = ref('aberta')
const selectedDemand = ref(null)

const visibleDemands = computed(() => demands.value.filter((demand) => !filter.value || demand.status === filter.value))

async function refresh() {
  await loadDemands(filter.value || null)
  selectedDemand.value = null
}

async function finishDemand() {
  if (!selectedDemand.value) return
  await completeDemand(selectedDemand.value.id)
  await refresh()
}

function printDemand() {
  window.print()
}

onMounted(refresh)
</script>

<template>
  <div class="page-shell demands-page">
    <header class="hero-card">
      <div>
        <p class="eyebrow">Operação</p>
        <h1>Demandas de separação</h1>
        <p class="subtitle">Acompanhe os produtos gerados pelas vendas e organize a retirada no estoque.</p>
      </div>
      <span class="badge demand-count">{{ visibleDemands.length }} demandas</span>
    </header>

    <section class="panel demand-toolbar">
      <label>
        Exibir demandas
        <select v-model="filter" @change="refresh">
          <option value="aberta">Em aberto</option>
          <option value="concluida">Concluídas</option>
          <option value="">Todas</option>
        </select>
      </label>
    </section>

    <section class="demand-layout">
      <div class="panel demand-list">
        <button v-for="demand in visibleDemands" :key="demand.id" type="button" class="demand-item" :class="{ selected: selectedDemand?.id === demand.id }" @click="selectedDemand = demand">
          <span><strong>Demanda #{{ demand.id }}</strong><small>Venda #{{ demand.sale_id }} · {{ demand.client_name }}</small></span>
          <span :class="['status-badge', demand.status === 'aberta' ? 'ativo' : 'inativo']">{{ demand.status === 'aberta' ? 'Em aberto' : 'Concluída' }}</span>
        </button>
        <p v-if="!visibleDemands.length" class="empty-state">Nenhuma demanda encontrada.</p>
      </div>

      <div v-if="selectedDemand" class="panel demand-detail">
        <div class="panel-header">
          <div><p class="eyebrow">Separação</p><h2>Demanda #{{ selectedDemand.id }}</h2></div>
          <div class="card-actions no-print"><button class="ghost-btn" type="button" @click="printDemand">Imprimir</button><button v-if="selectedDemand.status === 'aberta'" class="primary-btn" type="button" @click="finishDemand">Marcar concluída</button></div>
        </div>
        <p class="detail-meta">Venda #{{ selectedDemand.sale_id }} · Cliente: {{ selectedDemand.client_name }} · {{ selectedDemand.created_at }}</p>
        <div class="table-wrap">
          <table>
            <thead><tr><th>Produto</th><th>Código</th><th>Posição</th><th>Quantidade</th></tr></thead>
            <tbody><tr v-for="item in selectedDemand.items" :key="`${selectedDemand.id}-${item.code}`"><td>{{ item.name }}</td><td>{{ item.code }}</td><td>{{ item.position }}</td><td>{{ item.quantity }}</td></tr></tbody>
          </table>
        </div>
      </div>
      <div v-else class="panel demand-detail empty-detail"><h2>Selecione uma demanda</h2><p>Escolha uma demanda para visualizar os produtos e a posição de cada item.</p></div>
    </section>
  </div>
</template>

<style scoped>
.page-shell { display: flex; flex-direction: column; gap: 20px; }
.hero-card { background: linear-gradient(135deg, #0f172a 0%, #1d4ed8 100%); color: white; border-radius: 24px; padding: 24px; display: flex; justify-content: space-between; align-items: center; gap: 20px; }
.eyebrow { margin: 0 0 8px; font-size: 0.72rem; letter-spacing: 0.2em; text-transform: uppercase; opacity: 0.8; }
h1, h2 { margin: 0; }
.subtitle { margin: 8px 0 0; opacity: 0.9; }
.demand-toolbar label { max-width: 260px; display: flex; flex-direction: column; gap: 7px; font-weight: 700; }
.demand-toolbar select { border: 1px solid #dbe3f0; border-radius: 11px; padding: 10px 12px; background: #f8fafc; }
.demand-layout { display: grid; grid-template-columns: minmax(260px, 0.8fr) minmax(0, 1.7fr); gap: 20px; align-items: start; }
.demand-list { display: flex; flex-direction: column; gap: 8px; }
.demand-item { width: 100%; display: flex; justify-content: space-between; align-items: center; gap: 12px; text-align: left; border: 1px solid #e2e8f0; border-radius: 12px; padding: 13px; background: #f8fafc; cursor: pointer; }
.demand-item:hover, .demand-item.selected { border-color: #2563eb; background: #eff6ff; }
.demand-item span:first-child { display: flex; flex-direction: column; gap: 5px; }
.demand-item small, .detail-meta { color: #64748b; }
.demand-detail { min-height: 220px; }
.detail-meta { margin: 0 0 18px; }
.empty-detail { display: flex; flex-direction: column; justify-content: center; }
@media (max-width: 800px) { .demand-layout { grid-template-columns: 1fr; } .hero-card { align-items: flex-start; flex-direction: column; } }
@media print { .sidebar, .demand-toolbar, .demand-list, .no-print { display: none !important; } .content-area { padding: 0; } .demand-layout { display: block; } .demand-detail { box-shadow: none; border: 0; } }
</style>
