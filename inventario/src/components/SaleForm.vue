<script setup>
import { useInventory } from '../composables/useInventory'
const { saleForm, products, addSaleItem, removeSaleItem, handleSaleSubmit } = useInventory()
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
      <button type="button" class="secondary-btn" @click="addSaleItem">Adicionar produto</button>

      <div class="sale-items">
        <div v-for="item in saleForm.items" :key="item.product_id" class="sale-item">
          <span>{{ item.product_name }} <small>{{ item.quantity }} unidade(s)</small></span>
          <button type="button" class="danger-btn" @click="removeSaleItem(item.product_id)">Remover</button>
        </div>
        <p v-if="!saleForm.items.length" class="empty-state">Nenhum produto adicionado.</p>
      </div>
      <button type="submit" class="primary-btn">Confirmar venda</button>
    </form>
  </div>
</template>

<style scoped>
.sale-items { display: flex; flex-direction: column; gap: 8px; }
.sale-item { display: flex; justify-content: space-between; align-items: center; gap: 12px; padding: 10px 12px; border: 1px solid #dbe3f0; border-radius: 11px; background: #f8fafc; }
.sale-item span { min-width: 0; overflow-wrap: anywhere; }
.sale-item small { color: #64748b; }
.sale-item .danger-btn { min-height: 34px; padding: 7px 10px; }
</style>
