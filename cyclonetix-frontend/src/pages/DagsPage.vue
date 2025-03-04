<template>
  <q-page padding>
    <div class="q-mb-md">
      <div class="row items-center q-mb-lg">
        <div class="text-h5 q-mr-md">DAGs</div>
      </div>

      <search-filter
        :query="dagStore.query"
        @update:query="updateQuery"
      />

      <div class="row justify-between items-center q-mb-md">
        <div class="text-subtitle1">
          {{ dagStore.totalCount }} DAGs
        </div>
        <q-btn
          color="primary"
          icon="fas fa-code"
          label="Create DAG"
          no-caps
        />
      </div>

      <div v-if="dagStore.loading">
        <div class="text-center q-pa-xl">
          <q-spinner color="primary" size="3em" />
          <div class="q-mt-md text-grey">Loading DAGs...</div>
        </div>
      </div>

      <div v-else-if="dagStore.error" class="bg-red-1 text-red-9 q-pa-md rounded-borders">
        <div class="text-body1">Error loading DAGs: {{ dagStore.error }}</div>
      </div>

      <div v-else-if="!dagStore.hasDags" class="bg-blue-1 text-blue-9 q-pa-md rounded-borders">
        <div class="text-body1">No DAGs found matching your criteria.</div>
      </div>

      <div v-else>
        <q-card>
          <q-card-section>
            <q-table
              :rows="dagStore.dags"
              :columns="columns"
              row-key="dag_id"
              :pagination="pagination"
              @request="onRequest"
              :loading="dagStore.loading"
              class="dag-table"
              flat
              bordered
              :rows-per-page-options="[10, 20, 50, 0]"
              table-style="max-height: 70vh"
              virtual-scroll
            >
              <!-- DAG ID column with graph icon -->
              <template v-slot:body-cell-dag_id="props">
                <q-td :props="props">
                  <div class="row items-center">
                    <q-icon
                      name="fa-solid fa-diagram-project"
                      color="primary"
                      size="sm"
                      class="q-mr-sm cursor-pointer"
                      @click="goToGraph(props.row.dag_id)"
                    >
                      <q-tooltip>View DAG Graph</q-tooltip>
                    </q-icon>
                    <span>{{ props.row.dag_id }}</span>
                  </div>
                </q-td>
              </template>
              
              <!-- Status column -->
              <template v-slot:body-cell-status="props">
                <q-td :props="props">
                  <div class="row items-center">
                    <q-badge :color="getStatusColor(props.row)" class="status-badge">
                      {{ getStatusText(props.row) }}
                    </q-badge>
                  </div>
                </q-td>
              </template>
              
              <!-- Description column with tags -->
              <template v-slot:body-cell-description="props">
                <q-td :props="props">
                  <div>
                    {{ props.row.description || 'No description' }}
                    <div v-if="props.row.tags && props.row.tags.length > 0" class="q-mt-xs">
                      <q-chip
                        v-for="tag in props.row.tags"
                        :key="tag"
                        size="sm"
                        class="q-mr-xs"
                        dense
                      >
                        {{ tag }}
                      </q-chip>
                    </div>
                  </div>
                </q-td>
              </template>
              
              <!-- Date columns -->
              <template v-slot:body-cell-last_run="props">
                <q-td :props="props">
                  {{ formatDate(props.row.last_run) }}
                </q-td>
              </template>
              
              <template v-slot:body-cell-next_run="props">
                <q-td :props="props">
                  {{ formatDate(props.row.next_run) }}
                </q-td>
              </template>
              
              <!-- Runs column with counts -->
              <template v-slot:body-cell-runs="props">
                <q-td :props="props">
                  <div>
                    <span class="q-mr-sm">Total: {{ props.row.runs_count }}</span>
                    <span class="text-positive q-mr-sm">Success: {{ props.row.success_count }}</span>
                    <span class="text-negative q-mr-sm">Failed: {{ props.row.failed_count }}</span>
                    <span v-if="props.row.running_count > 0" class="text-info">
                      Running: {{ props.row.running_count }}
                    </span>
                  </div>
                </q-td>
              </template>
              
              <!-- Actions column with vertical layout -->
              <template v-slot:body-cell-actions="props">
                <q-td :props="props">
                  <div class="column items-center q-gutter-y-sm">
                    <q-btn
                      :color="props.row.paused ? 'positive' : 'warning'"
                      :icon="props.row.paused ? 'fas fa-play' : 'fas fa-pause'"
                      size="sm"
                      round
                      flat
                      @click="toggleDagPaused({ dagId: props.row.dag_id, paused: !props.row.paused })"
                    >
                      <q-tooltip>{{ props.row.paused ? 'Unpause' : 'Pause' }}</q-tooltip>
                    </q-btn>
                    
                    <q-btn
                      color="primary"
                      icon="fas fa-play"
                      size="sm"
                      round
                      flat
                    >
                      <q-tooltip>Trigger</q-tooltip>
                    </q-btn>
                    
                    <q-btn 
                      flat 
                      round
                      size="sm"
                      color="primary" 
                      icon="fas fa-code"
                    >
                      <q-tooltip>View Code</q-tooltip>
                    </q-btn>
                    
                    <q-btn 
                      flat 
                      round
                      size="sm"
                      color="primary" 
                      icon="fas fa-history"
                    >
                      <q-tooltip>View History</q-tooltip>
                    </q-btn>
                  </div>
                </q-td>
              </template>
            </q-table>
          </q-card-section>
        </q-card>

      </div>
    </div>
  </q-page>
</template>

<script>
import { onMounted, computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useDagStore } from '../store/dag-module'
import SearchFilter from '../components/SearchFilter.vue'

export default {
  name: 'DagsPage',

  components: {
    SearchFilter
  },

  setup() {
    const router = useRouter()
    const dagStore = useDagStore()
    const currentPage = computed({
      get: () => dagStore.query.page,
      set: (value) => dagStore.setPage(value)
    })
    
    // Table pagination state
    const pagination = ref({
      sortBy: 'dag_id',
      descending: false,
      page: 1,
      rowsPerPage: 10
    })
    
    // Define table columns
    const columns = [
      { 
        name: 'dag_id', 
        align: 'left', 
        label: 'DAG ID', 
        field: 'dag_id', 
        sortable: true,
        style: 'width: 200px'
      },
      { 
        name: 'status', 
        align: 'left', 
        label: 'Status', 
        field: row => getStatusText(row), 
        sortable: true,
        style: 'width: 120px'
      },
      { 
        name: 'description', 
        align: 'left', 
        label: 'Description', 
        field: 'description', 
        sortable: true 
      },
      { 
        name: 'owner', 
        align: 'left', 
        label: 'Owner', 
        field: 'owner', 
        sortable: true,
        style: 'width: 150px'
      },
      { 
        name: 'schedule_interval', 
        align: 'left', 
        label: 'Schedule', 
        field: 'schedule_interval', 
        sortable: true,
        style: 'width: 150px'
      },
      { 
        name: 'last_run', 
        align: 'left', 
        label: 'Last Run', 
        field: 'last_run', 
        sortable: true,
        style: 'width: 180px'
      },
      { 
        name: 'next_run', 
        align: 'left', 
        label: 'Next Run', 
        field: 'next_run', 
        sortable: true,
        style: 'width: 180px'
      },
      { 
        name: 'runs', 
        align: 'left', 
        label: 'Runs', 
        field: row => row.runs_count, 
        sortable: true,
        style: 'width: 300px'
      },
      { 
        name: 'actions', 
        align: 'center', 
        label: 'Actions', 
        field: 'actions',
        style: 'width: 80px'
      }
    ]

    onMounted(() => {
      dagStore.fetchDags()
    })
    
    // Format date for display
    const formatDate = (dateString) => {
      if (!dateString) return 'N/A'
      const date = new Date(dateString)
      return date.toLocaleString()
    }
    
    // Get status color based on DAG state
    const getStatusColor = (dag) => {
      if (dag.paused) return 'warning'
      if (dag.running_count > 0) return 'info'
      if (dag.failed_count > 0) return 'negative'
      if (dag.success_count > 0) return 'positive'
      return 'grey'
    }
    
    // Get status text based on DAG state
    const getStatusText = (dag) => {
      if (dag.paused) return 'Paused'
      if (dag.running_count > 0) return 'Running'
      if (dag.failed_count > 0) return 'Failed'
      if (dag.success_count > 0) return 'Success'
      return 'No Status'
    }

    // Handle filter query updates
    const updateQuery = (newQuery) => {
      dagStore.setQuery(newQuery)
    }

    // Toggle DAG paused state
    const toggleDagPaused = ({ dagId, paused }) => {
      dagStore.toggleDagPaused(dagId, paused)
    }

    // Handle page changes
    const onPageChange = (page) => {
      dagStore.setPage(page)
    }
    
    // Handle table request changes
    const onRequest = (props) => {
      const { page, rowsPerPage, sortBy, descending } = props.pagination
      
      pagination.value = {
        page,
        rowsPerPage,
        sortBy, 
        descending
      }
      
      // Update the store page if changed
      if (page !== dagStore.query.page) {
        dagStore.setPage(page)
      }
    }
    
    // Navigate to the graph page
    const goToGraph = (dagId) => {
      router.push({ name: 'dag-graph', params: { dagId } })
    }

    return {
      dagStore,
      columns,
      pagination,
      currentPage,
      updateQuery,
      toggleDagPaused,
      onPageChange,
      onRequest,
      formatDate,
      getStatusColor,
      getStatusText,
      goToGraph
    }
  }
}
</script>

<style>
.q-table__top {
  padding-top: 8px;
  padding-bottom: 8px;
}

.dag-table tbody td {
  padding-top: 12px;
  padding-bottom: 12px;
}

.dag-table .q-chip {
  font-size: 11px;
}

/* Status circle styling */
.status-badge {
  border-radius: 4px;
  font-weight: 500;
  padding: 4px 8px;
}

/* Hover effect for rows */
.dag-table tbody tr:hover {
  background-color: rgba(0, 0, 0, 0.03);
}

/* Dark mode adjustments */
.body--dark .dag-table tbody tr:hover {
  background-color: rgba(255, 255, 255, 0.07);
}
</style>
