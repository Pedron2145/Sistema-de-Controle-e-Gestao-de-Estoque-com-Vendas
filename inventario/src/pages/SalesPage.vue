<script setup>
import HeaderStats from '../components/HeaderStats.vue'
import Feedback from '../components/Feedback.vue'
import SaleForm from '../components/SaleForm.vue'
import SummaryPanel from '../components/SummaryPanel.vue'
import { useAuth } from '../composables/useAuth'
import { onMounted } from 'vue'
import { useInventory } from '../composables/useInventory'

const { hasPermission } = useAuth()
const { loadData } = useInventory()
const canSell = hasPermission('vendas', 'can_create')

onMounted(() => {
  loadData()
})
</script>

<template>
  <div class="page-shell">
    <HeaderStats />
    <Feedback />

    <section class="grid">
      <div v-if="canSell" class="sales-form-wrap">
        <SaleForm />
      </div>
      <div v-else class="panel readonly-panel">
        <h2>Vendas bloqueadas</h2>
        <p>Seu perfil não possui permissão para registrar vendas.</p>
      </div>
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
  grid-template-columns: 1.2fr 1fr;
  gap: 20px;
}

.sales-form-wrap {
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
