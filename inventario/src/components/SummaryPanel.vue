<script setup>
import { useInventory } from '../composables/useInventory'
const { lowStockProducts } = useInventory()
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>Resumo</h2>
      <span class="badge danger">{{ lowStockProducts.length }} com estoque baixo</span>
    </div>

    <div v-if="lowStockProducts.length" class="warning-list">
      <p v-for="product in lowStockProducts" :key="product.id">{{ product.name }} · {{ product.quantity }} unidade(s)</p>
    </div>
    <p v-else class="empty-state">Estoque saudável para todos os produtos.</p>

  </div>
</template>

<style scoped></style>
