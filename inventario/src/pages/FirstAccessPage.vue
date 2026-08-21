<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuth } from '../composables/useAuth'

const { registerFirstAdmin, error, loading } = useAuth()
const router = useRouter()

const form = ref({
  name: '',
  email: '',
  password: '',
})

async function onSubmit() {
  try {
    await registerFirstAdmin(form.value)
    router.push('/estoque')
  } catch {
    // erro já exposto no composable
  }
}
</script>

<template>
  <div class="auth-shell">
    <div class="auth-card">
      <div class="brand-block">
        <div class="brand-mark">ERP</div>
        <div>
          <p class="eyebrow">Primeiro acesso</p>
          <h1>Criar administrador</h1>
        </div>
      </div>

      <p class="intro">
        Este é o cadastro inicial do sistema. Depois disso, a criação de novos usuários será feita apenas pelo administrador.
      </p>

      <form class="auth-form" @submit.prevent="onSubmit">
        <label>
          Nome completo
          <input v-model="form.name" type="text" placeholder="Seu nome" required />
        </label>

        <label>
          E-mail
          <input v-model="form.email" type="email" placeholder="admin@empresa.com" required />
        </label>

        <label>
          Senha
          <input v-model="form.password" type="password" placeholder="Crie uma senha forte" required />
        </label>

        <p v-if="error" class="error-text">{{ error }}</p>

        <button type="submit" class="primary-btn" :disabled="loading">
          {{ loading ? 'Criando...' : 'Criar administrador' }}
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.auth-shell {
  min-height: 100vh;
  display: grid;
  place-items: center;
  background: linear-gradient(135deg, #edf6ff 0%, #eef4ff 100%);
}

.auth-card {
  width: min(460px, 92vw);
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid #dbe7ff;
  border-radius: 24px;
  box-shadow: 0 28px 60px rgba(15, 23, 42, 0.12);
  padding: 30px 26px;
}

.brand-block {
  display: flex;
  align-items: center;
  gap: 14px;
  margin-bottom: 18px;
}

.brand-mark {
  display: grid;
  place-items: center;
  width: 54px;
  height: 54px;
  border-radius: 16px;
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  font-weight: 800;
}

.eyebrow {
  margin: 0;
  text-transform: uppercase;
  letter-spacing: 0.18em;
  color: #64748b;
  font-size: 0.7rem;
}

h1 {
  margin: 6px 0 0;
  font-size: 1.7rem;
}

.intro {
  margin: 0 0 18px;
  color: #475569;
  line-height: 1.5;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

label {
  display: flex;
  flex-direction: column;
  gap: 8px;
  font-weight: 600;
  color: #1e293b;
}

input {
  border: 1px solid #dbe3f0;
  border-radius: 12px;
  background: #f8fafc;
  padding: 12px 14px;
  font: inherit;
}

.primary-btn {
  border: none;
  border-radius: 12px;
  padding: 12px 16px;
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  font-weight: 700;
  cursor: pointer;
}

.primary-btn:disabled {
  opacity: 0.75;
  cursor: wait;
}

.error-text {
  margin: 0;
  color: #b91c1c;
  font-size: 0.92rem;
}
</style>
