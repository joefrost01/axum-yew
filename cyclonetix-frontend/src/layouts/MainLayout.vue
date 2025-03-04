<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated class="bg-blue-grey-8">
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />
        <q-img
          src="/logo.png"
          spinner-color="white"
          style="height: 40px; max-width: 40px;"
          class="q-mr-md"
        />
        <q-toolbar-title>Cyclonetix</q-toolbar-title>

        <q-btn
          flat
          round
          dense
          :icon="isDark ? 'dark_mode' : 'light_mode'"
          @click="toggleDarkMode"
          :aria-label="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
        >
          <q-tooltip>{{ isDark ? 'Switch to light mode' : 'Switch to dark mode' }}</q-tooltip>
        </q-btn>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label header>Navigation</q-item-label>

        <q-item clickable v-ripple to="/">
          <q-item-section avatar>
            <q-icon name="home" />
          </q-item-section>
          <q-item-section>Home</q-item-section>
        </q-item>

        <q-item clickable v-ripple to="/dags">
          <q-item-section avatar>
            <q-icon name="fas fa-project-diagram" />
          </q-item-section>
          <q-item-section>DAGs</q-item-section>
        </q-item>
      </q-list>
    </q-drawer>

    <q-page-container>
      <!-- Render page content (e.g. DagsPage) here -->
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { ref, onMounted, watch } from 'vue'
import { useQuasar } from 'quasar'

export default {
  name: "MainLayout",

  setup() {
    const $q = useQuasar()
    const leftDrawerOpen = ref(false)
    const isDark = ref(false)

    // Check system preference and localStorage on mount
    onMounted(() => {
      // First check localStorage
      const storedDarkMode = localStorage.getItem('darkMode')
      if (storedDarkMode !== null) {
        isDark.value = storedDarkMode === 'true'
      } else {
        // If not in localStorage, use system preference
        isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
      }

      // Set initial theme
      $q.dark.set(isDark.value)
    })

    // Watch for changes to isDark and update theme accordingly
    watch(isDark, val => {
      $q.dark.set(val)
      localStorage.setItem('darkMode', val.toString())
    })

    const toggleDarkMode = () => {
      isDark.value = !isDark.value
    }

    return {
      leftDrawerOpen,
      isDark,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value
      },
      toggleDarkMode
    }
  }
};
</script>

<style>
/* Styles for your layout */
</style>
