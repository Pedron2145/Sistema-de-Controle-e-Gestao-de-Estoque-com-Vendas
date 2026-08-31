import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const session = ref(null)
const loading = ref(false)
const error = ref('')

const isAuthenticated = computed(() => Boolean(session.value?.token))
const user = computed(() => session.value?.user ?? null)
const permissions = computed(() => session.value?.permissions ?? [])

function hasPermission(moduleName, action = 'can_view') {
  const currentUser = user.value
  if (!currentUser) return false
  if (currentUser.role === 'admin') return true

  const modulePermission = permissions.value.find((item) => item.module === moduleName)
  return Boolean(modulePermission?.[action])
}

function setSession(nextSession) {
  session.value = nextSession
}

async function checkFirstAccess() {
  try {
    const status = await invoke('app_status')
    return status
  } catch (err) {
    error.value = err?.toString() ?? 'Não foi possível verificar o estado do sistema.'
    return null
  }
}

async function registerFirstAdmin({ name, email, password }) {
  loading.value = true
  error.value = ''

  try {
    const auth = await invoke('register_first_admin', { payload: { name, email, password } })
    setSession(auth)
    return auth
  } catch (err) {
    error.value = err?.toString() ?? 'Não foi possível criar o administrador.'
    throw err
  } finally {
    loading.value = false
  }
}

async function login({ email, password }) {
  loading.value = true
  error.value = ''

  try {
    const auth = await invoke('login', { payload: { email, password } })
    setSession(auth)
    return auth
  } catch (err) {
    error.value = err?.toString() ?? 'Não foi possível entrar no sistema.'
    throw err
  } finally {
    loading.value = false
  }
}

async function listUsers() {
  if (!session.value?.token) return []

  try {
    return await invoke('list_users', { session_token: session.value.token })
  } catch (err) {
    error.value = err?.toString() ?? 'Não foi possível listar usuários.'
    return []
  }
}

async function createUser({ name, email, password, role, permissions }) {
  if (!session.value?.token) {
    throw new Error('Sessão de administrador não encontrada.')
  }

  loading.value = true
  error.value = ''

  try {
    return await invoke('create_user', {
      payload: {
        name,
        email,
        password,
        role,
        session_token: session.value.token,
        permissions,
      },
    })
  } catch (err) {
    error.value = err?.toString() ?? 'Não foi possível criar o usuário.'
    throw err
  } finally {
    loading.value = false
  }
}

async function listPermissions(userId = null) {
  if (!session.value?.token) return []

  try {
    return await invoke('list_user_permissions', {
      session_token: session.value.token,
      user_id: userId,
    })
  } catch (err) {
    error.value = err?.toString() ?? 'Não foi possível carregar permissões.'
    return []
  }
}

async function updatePermissions({ userId, permissions }) {
  if (!session.value?.token) {
    throw new Error('Sessão de administrador não encontrada.')
  }

  return await invoke('update_user_permissions', {
    payload: {
      session_token: session.value.token,
      user_id: userId,
      permissions,
    },
  })
}

async function logout() {
  if (!session.value?.token) {
    setSession(null)
    return
  }

  try {
    await invoke('logout', { session_token: session.value.token })
  } catch {
    // Ignora falhas na sessão e limpa localmente
  } finally {
    setSession(null)
  }
}

export function useAuth() {
  return {
    session,
    loading,
    error,
    user,
    permissions,
    isAuthenticated,
    hasPermission,
    checkFirstAccess,
    registerFirstAdmin,
    login,
    listUsers,
    createUser,
    listPermissions,
    updatePermissions,
    logout,
  }
}
