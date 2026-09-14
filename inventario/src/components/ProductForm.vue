<script setup>
import { useInventory } from '../composables/useInventory'
import { useAuth } from '../composables/useAuth'
const { productForm, editingProductId, handleProductSubmit, resetProductForm } = useInventory()
const { hasPermission } = useAuth()
const canCreate = hasPermission('estoque', 'can_create')
const canEdit = hasPermission('estoque', 'can_edit')
</script>

<template>
  <div class="panel">
    <div class="panel-header">
      <h2>{{ editingProductId ? 'Editar produto' : 'Adicionar produto' }}</h2>
      <button v-if="editingProductId" class="ghost-btn" @click="resetProductForm">Cancelar</button>
    </div>

    <form class="form-stack" @submit.prevent="handleProductSubmit">
      <label>
        Código do produto
        <input v-model="productForm.code" placeholder="Ex.: TEC-001" required />
      </label>
      <label>
        Nome do produto
        <input v-model="productForm.name" placeholder="Ex.: Teclado mecânico" required />
      </label>
      <label>
        Fabricante
        <input v-model="productForm.manufacturer" placeholder="Ex.: TechLabs" />
      </label>
      <label>
        Fornecedor
        <input v-model="productForm.supplier" placeholder="Ex.: Distribuidora Tech" required />
      </label>
      <label>
        Fabricante
        <input v-model="productForm.manufacturer" placeholder="Ex.: TechLabs" />
      </label>
      <label>
        Marca
        <input v-model="productForm.brand" placeholder="Ex.: HyperKey" />
      </label>
      <label>
        Status
        <select v-model="productForm.status">
          <option value="ativo">Ativo</option>
          <option value="inativo">Inativo</option>
        </select>
      </label>
      <div class="position-grid">
        <label>Rua<input v-model.number="productForm.street" type="number" min="0" max="99" /></label>
        <label>Posição<input v-model.number="productForm.position" type="number" min="0" max="99" /></label>
        <label>Nível<input v-model.number="productForm.level" type="number" min="0" max="99" /></label>
        <label>Apartamento<input v-model.number="productForm.apartment" type="number" min="0" max="99" /></label>
      </div>
      <label>
        Quantidade
        <input v-model.number="productForm.quantity" type="number" min="0" />
      </label>
      <button v-if="editingProductId ? canEdit : canCreate" type="submit" class="primary-btn">{{ editingProductId ? 'Salvar alterações' : 'Cadastrar produto' }}</button>
    </form>
  </div>
</template>

<style scoped></style>
