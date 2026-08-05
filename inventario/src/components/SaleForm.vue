<script setup>
import { useInventory } from '../composables/useInventory'
const { saleForm, products, handleSaleSubmit } = useInventory()
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>Registrar venda</h2>
      <span class="badge">Baixa no estoque</span>
    </div>

    <form class="form-stack" @submit.prevent="handleSaleSubmit">
      <label>
        Produto
        <select v-model="saleForm.productId">
          <option value="" disabled>Selecione um produto</option>
          <option v-for="product in products" :key="product.id" :value="product.id">{{ product.name }} (estoque: {{ product.quantity }})</option>
        </select>
      </label>
      <label>
        Cliente / Empresa
        <input v-model="saleForm.clientName" placeholder="Ex.: João da Silva" />
      </label>
      <label>
        Tipo de cliente
        <select v-model="saleForm.customerType">
          <option value="Pessoa física">Pessoa física</option>
          <option value="Empresa">Empresa</option>
        </select>
      </label>
      <label>
        Quantidade da venda
        <input v-model.number="saleForm.quantity" type="number" min="1" />
      </label>
      <button type="submit" class="primary-btn">Confirmar venda</button>
    </form>
  </div>
</template>

<style scoped></style>
