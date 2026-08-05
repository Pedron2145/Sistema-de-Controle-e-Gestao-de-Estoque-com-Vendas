<script setup>
import { useInventory } from '../composables/useInventory'
const { feedback } = useInventory()
</script>

<template>
  <div v-if="feedback.message" :class="['feedback', feedback.type]">
    {{ feedback.message }}
  </div>
</template>

<style scoped></style>
