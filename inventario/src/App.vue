<script setup>
import { onMounted } from 'vue'
import HeaderStats from './components/HeaderStats.vue'
import Feedback from './components/Feedback.vue'
import ProductForm from './components/ProductForm.vue'
import ProductList from './components/ProductList.vue'
import SaleForm from './components/SaleForm.vue'
import SummaryPanel from './components/SummaryPanel.vue'
import DbActions from './components/DbActions.vue'
import { useInventory } from './composables/useInventory'

const { resetSaleForm } = useInventory()

onMounted(() => {
  resetSaleForm()
})
</script>

<template>
  <div class="app-shell">
    <HeaderStats />
    <Feedback />

    <section class="grid">
      <ProductForm />
      <ProductList />
    </section>

    <section class="grid">
      <SaleForm />
      <SummaryPanel />
    </section>

    <DbActions />
  </div>
</template>

<style>
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
