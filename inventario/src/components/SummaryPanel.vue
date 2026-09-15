<script setup>
import { useInventory } from '../composables/useInventory'
const { totalProducts, totalStock, lowStockProducts } = useInventory()
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>Resumo</h2>
      <span class="badge danger">{{ lowStockProducts.length }} com estoque baixo</span>
    </div>

    <div class="stock-summary">
      <div class="summary-stat">
        <strong>{{ totalProducts }}</strong>
        <span>Produtos cadastrados</span>
      </div>
      <div class="summary-stat">
        <strong>{{ totalStock }}</strong>
        <span>Unidades em estoque</span>
      </div>
    </div>

    <div v-if="lowStockProducts.length" class="warning-list">
      <p v-for="product in lowStockProducts" :key="product.id">{{ product.name }} · {{ product.quantity }} unidade(s)</p>
    </div>
    <p v-else class="empty-state">Estoque saudável para todos os produtos.</p>

  </div>
</template>

<style scoped>
.stock-summary {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
  margin-bottom: 16px;
}

.summary-stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px;
  border: 1px solid #dbe3f0;
  border-radius: 11px;
  background: #f8fafc;
}

.summary-stat strong {
  color: #0f172a;
  font-size: 1.35rem;
}

.summary-stat span {
  color: #64748b;
  font-size: 0.85rem;
}
</style>
