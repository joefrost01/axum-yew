<template>
  <div class="q-mb-md">
    <q-card flat bordered class="q-pa-md">
      <div class="row q-col-gutter-md">
        <div class="col-12 col-md-4">
          <q-input
              v-model="searchText"
              outlined
              dense
              label="Search DAGs"
              placeholder="Search by DAG ID or owner"
              @update:model-value="onSearchChange"
              clearable
          >
            <template v-slot:prepend>
              <q-icon name="fas fa-search" />
            </template>
          </q-input>
        </div>

        <div class="col-12 col-md-4">
          <q-select
              v-model="statusFilter"
              :options="statusOptions"
              outlined
              dense
              label="Status"
              @update:model-value="onStatusChange"
              clearable
              emit-value
              map-options
          >
            <template v-slot:prepend>
              <q-icon name="fas fa-filter" />
            </template>
          </q-select>
        </div>

        <div class="col-12 col-md-4">
          <q-input
              v-model="tagsFilter"
              outlined
              dense
              label="Tags (comma separated)"
              placeholder="e.g. production,etl"
              @update:model-value="onTagsChange"
              clearable
          >
            <template v-slot:prepend>
              <q-icon name="fas fa-tags" />
            </template>
          </q-input>
        </div>
      </div>

      <div class="row q-col-gutter-md q-mt-md">
        <div class="col-12 col-md-4">
          <q-select
              v-model="sortBy"
              :options="sortOptions"
              outlined
              dense
              label="Sort by"
              @update:model-value="onSortChange"
              clearable
              emit-value
              map-options
          >
            <template v-slot:prepend>
              <q-icon name="fas fa-sort" />
            </template>
          </q-select>
        </div>

        <div class="col-12 col-md-4">
          <q-select
              v-model="sortOrder"
              :options="orderOptions"
              outlined
              dense
              label="Sort order"
              @update:model-value="onOrderChange"
              clearable
              emit-value
              map-options
          >
            <template v-slot:prepend>
              <q-icon name="fas fa-sort-amount-down" />
            </template>
          </q-select>
        </div>

        <div class="col-12 col-md-4">
          <q-btn
              color="primary"
              label="Reset Filters"
              icon="fas fa-undo"
              no-caps
              flat
              class="full-width"
              @click="resetFilters"
          />
        </div>
      </div>
    </q-card>
  </div>
</template>

<script>
import { ref, watch, onMounted } from 'vue'

export default {
  name: 'SearchFilter',

  props: {
    query: {
      type: Object,
      required: true
    }
  },

  emits: ['update:query'],

  setup(props, { emit }) {
    // Form state
    const searchText = ref('')
    const statusFilter = ref(null)
    const tagsFilter = ref('')
    const sortBy = ref(null)
    const sortOrder = ref(null)

    // Options for dropdowns
    const statusOptions = [
      { label: 'All', value: 'all' },
      { label: 'Active', value: 'active' },
      { label: 'Paused', value: 'paused' },
      { label: 'Success', value: 'success' },
      { label: 'Failed', value: 'failed' },
      { label: 'Running', value: 'running' }
    ]

    const sortOptions = [
      { label: 'DAG ID', value: 'dag_id' },
      { label: 'Owner', value: 'owner' },
      { label: 'Last Run', value: 'last_run' },
      { label: 'Next Run', value: 'next_run' },
      { label: 'Status', value: 'status' }
    ]

    const orderOptions = [
      { label: 'Ascending', value: 'asc' },
      { label: 'Descending', value: 'desc' }
    ]

    // Initialize from props
    onMounted(() => {
      searchText.value = props.query.search || ''
      statusFilter.value = props.query.status || null
      tagsFilter.value = props.query.tags || ''
      sortBy.value = props.query.sort_by || null
      sortOrder.value = props.query.sort_order || null
    })

    // Watch for external query changes
    watch(() => props.query, (newQuery) => {
      searchText.value = newQuery.search || ''
      statusFilter.value = newQuery.status || null
      tagsFilter.value = newQuery.tags || ''
      sortBy.value = newQuery.sort_by || null
      sortOrder.value = newQuery.sort_order || null
    }, { deep: true })

    // Debounce search to avoid too many API calls
    let searchTimeout = null
    const onSearchChange = (value) => {
      if (searchTimeout) clearTimeout(searchTimeout)

      searchTimeout = setTimeout(() => {
        updateQuery({ search: value || null })
      }, 300)
    }

    const onStatusChange = (value) => {
      updateQuery({ status: value === 'all' ? null : value })
    }

    const onTagsChange = (value) => {
      updateQuery({ tags: value || null })
    }

    const onSortChange = (value) => {
      updateQuery({ sort_by: value })
    }

    const onOrderChange = (value) => {
      updateQuery({ sort_order: value })
    }

    const resetFilters = () => {
      searchText.value = ''
      statusFilter.value = null
      tagsFilter.value = ''
      sortBy.value = null
      sortOrder.value = null

      updateQuery({
        search: null,
        status: null,
        tags: null,
        sort_by: null,
        sort_order: null
      })
    }

    const updateQuery = (updates) => {
      emit('update:query', { ...props.query, ...updates })
    }

    return {
      searchText,
      statusFilter,
      tagsFilter,
      sortBy,
      sortOrder,
      statusOptions,
      sortOptions,
      orderOptions,
      onSearchChange,
      onStatusChange,
      onTagsChange,
      onSortChange,
      onOrderChange,
      resetFilters
    }
  }
}
</script>