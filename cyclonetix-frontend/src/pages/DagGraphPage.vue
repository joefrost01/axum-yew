<template>
  <q-page padding>
    <div class="q-mb-md">
      <div class="row items-center q-mb-lg">
        <div class="text-h5 q-mr-md">DAG Graph: {{ dagId }}</div>
        <q-btn
          flat
          color="primary"
          icon="fas fa-arrow-left"
          label="Back to DAGs"
          :to="{ name: 'dags' }"
          no-caps
        />
      </div>

      <div v-if="loading">
        <div class="text-center q-pa-xl">
          <q-spinner color="primary" size="3em" />
          <div class="q-mt-md text-grey">Loading DAG Graph...</div>
        </div>
      </div>

      <div v-else-if="error" class="bg-red-1 text-red-9 q-pa-md rounded-borders">
        <div class="text-body1">Error loading DAG Graph: {{ error }}</div>
      </div>

      <div v-else>
        <div class="row q-mb-md">
          <div class="col-12">
            <q-card>
              <q-card-section>
                <div class="text-subtitle1 q-mb-sm">DAG Graph Visualization</div>
                <div id="dag-graph" class="dag-cytoscape-container perspective-container"></div>
              </q-card-section>
            </q-card>
          </div>
        </div>

        <q-card class="q-mt-md">
          <q-card-section>
            <div class="row items-center q-mb-md">
              <div class="text-subtitle1 q-mr-auto">Tasks</div>
              <q-input
                v-model="taskSearchQuery"
                dense
                outlined
                placeholder="Search tasks..."
                class="col-grow"
                clearable
                @update:model-value="updateTaskFilter"
              >
                <template v-slot:prepend>
                  <q-icon name="search" />
                </template>
                <template v-slot:append v-if="taskSearchQuery">
                  <q-icon name="clear" class="cursor-pointer" @click="taskSearchQuery = ''" />
                </template>
              </q-input>
            </div>

            <q-table
              :rows="filteredTasks"
              :columns="taskColumns"
              row-key="id"
              v-model:pagination="taskTablePagination"
              :filter="taskSearchQuery"
              @request="onTaskTableRequest"
              virtual-scroll
              style="height: 40vh"
              :rows-per-page-options="[0, 10, 20, 50]"
              color="primary"
              table-header-class="text-primary"
              :loading="loading"
              :selected-rows-label="getSelectedString"
              selection="single"
              v-model:selected="selectedTask"
              flat
              bordered
            >
              <template v-slot:body-cell-status="props">
                <q-td :props="props">
                  <q-badge :color="getStatusColor(props.row.status)">
                    {{ props.row.status }}
                  </q-badge>
                </q-td>
              </template>
              <template v-slot:body-cell-duration="props">
                <q-td :props="props">
                  {{ props.row.duration ? formatDuration(props.row.duration) : 'N/A' }}
                </q-td>
              </template>
              <template v-slot:body-cell-start_time="props">
                <q-td :props="props">
                  {{ props.row.start_time ? formatDate(props.row.start_time) : 'N/A' }}
                </q-td>
              </template>
              <template v-slot:body-cell-end_time="props">
                <q-td :props="props">
                  {{ props.row.end_time ? formatDate(props.row.end_time) : 'N/A' }}
                </q-td>
              </template>
              <template v-slot:body-cell-retries="props">
                <q-td :props="props">
                  {{ props.row.retries }} / {{ props.row.max_retries }}
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
import { ref, onMounted, onBeforeUnmount, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useDagStore } from '../store/dag-module'
import cytoscape from 'cytoscape'
import dagre from 'cytoscape-dagre'

// Register the dagre layout with cytoscape
cytoscape.use(dagre)

export default {
  name: 'DagGraphPage',

  setup() {
    const route = useRoute()
    const dagStore = useDagStore()
    const dagId = computed(() => route.params.dagId)
    const graph = ref(null)
    const loading = ref(true)
    const error = ref(null)
    const taskSearchQuery = ref('')
    const selectedTask = ref([]) // For table row selection
    const taskTablePagination = ref({
      sortBy: 'id',
      descending: false,
      page: 1,
      rowsPerPage: 10,
      rowsNumber: 0
    })
    let cy = null

    const taskColumns = [
      { name: 'id', align: 'left', label: 'ID', field: 'id', sortable: true },
      { name: 'name', align: 'left', label: 'Name', field: 'name', sortable: true },
      { name: 'status', align: 'left', label: 'Status', field: 'status', sortable: true },
      { name: 'operator', align: 'left', label: 'Operator', field: 'operator', sortable: true },
      { name: 'duration', align: 'left', label: 'Duration', field: 'duration', sortable: true },
      { name: 'start_time', align: 'left', label: 'Start Time', field: 'start_time', sortable: true },
      { name: 'end_time', align: 'left', label: 'End Time', field: 'end_time', sortable: true },
      { name: 'retries', align: 'left', label: 'Retries', field: 'retries', sortable: true }
    ]

    const getStatusColor = (status) => {
      switch (status) {
        case 'SUCCEEDED': return 'positive'
        case 'RUNNING': return 'warning'  // Orange
        case 'FAILED': return 'negative'
        case 'SKIPPED': return 'grey'
        case 'QUEUED': return 'info'      // Cyan
        case 'PENDING': return 'info'     // Cyan
        case 'PAUSED': return 'grey-7'
        default: return 'grey'
      }
    }

    const formatDate = (dateString) => {
      if (!dateString) return 'N/A'
      const date = new Date(dateString)
      return date.toLocaleString()
    }

    const formatDuration = (durationInSeconds) => {
      if (durationInSeconds < 60) {
        return `${durationInSeconds.toFixed(1)}s`
      } else if (durationInSeconds < 3600) {
        const minutes = Math.floor(durationInSeconds / 60)
        const seconds = Math.round(durationInSeconds % 60)
        return `${minutes}m ${seconds}s`
      } else {
        const hours = Math.floor(durationInSeconds / 3600)
        const minutes = Math.floor((durationInSeconds % 3600) / 60)
        return `${hours}h ${minutes}m`
      }
    }

    const fetchDagGraph = async () => {
      loading.value = true
      error.value = null

      try {
        const data = await dagStore.fetchDagGraph(dagId.value)
        graph.value = data
        initCytoscape()
      } catch (err) {
        console.error('Error fetching DAG graph:', err)
        error.value = err.message || 'Error fetching DAG graph'
      } finally {
        loading.value = false
      }
    }

    const initCytoscape = () => {
      if (!graph.value) return

      // Destroy previous instance if it exists
      if (cy) {
        cy.destroy()
      }

      // Create nodes and edges from the graph data
      const elements = {
        nodes: graph.value.tasks.map(task => ({
          data: {
            id: task.id,
            // Just use original name + operator, we'll handle wrapping with CSS
            label: `${task.name}\n${task.operator}`,
            name: task.name, // Keep original name separately
            status: task.status,
            operator: task.operator
          }
        })),
        edges: graph.value.edges.map(edge => ({
          data: {
            id: `${edge.source}-${edge.target}`,
            source: edge.source,
            target: edge.target
          }
        }))
      }

      // Wait for the DOM to be ready
      setTimeout(() => {
        const container = document.getElementById('dag-graph')
        if (!container) return

        // Enable 3D perspective in container
        container.style.transform = 'perspective(1000px)';

        // Custom text-wrapping function for Cytoscape
        function wrapNodeLabels() {
          cy.nodes().forEach(node => {
            const name = node.data('name');
            const operator = node.data('operator');

            // Replace underscores with a special character that's treated like a space for wrapping
            const processedName = name.replace(/_/g, '_ ').trim();

            // Set the processed label
            node.data('label', `${processedName}\n${operator}`);
          });
        }

        cy = cytoscape({
          container,
          elements,
          style: [
            {
              selector: 'node',
              style: {
                'background-color': '#9AA0A6', // Grey as default
                'label': 'data(label)',
                'color': '#fff',
                'text-valign': 'center',
                'text-halign': 'center',
                'font-size': '12px',
                'width': '220px',
                'height': '70px',          // Reasonable height for two lines
                'shape': 'roundrectangle',
                'text-wrap': 'wrap',
                'text-max-width': '200px',  // More width for text
                'text-margin-y': '5px',     // Vertical margin
                'line-height': '1.3',       // Better line height
                'text-overflow-wrap': 'anywhere', // Allow breaking at any character
                'word-break': 'break-word'  // Enable breaking within words
              }
            },
            // SUCCEEDED - Green
            {
              selector: 'node[status="SUCCEEDED"]',
              style: {
                'background-color': '#21BA45' // Green for succeeded
              }
            },
            // FAILED - Red
            {
              selector: 'node[status="FAILED"]',
              style: {
                'background-color': '#C10015' // Red for failed
              }
            },
            // RUNNING - Orange
            {
              selector: 'node[status="RUNNING"]',
              style: {
                'background-color': '#F2C037' // Orange for running
              }
            },
            // QUEUED - Cyan
            {
              selector: 'node[status="QUEUED"]',
              style: {
                'background-color': '#31CCEC' // Cyan for queued
              }
            },
            // PENDING - Cyan (similar to queued)
            {
              selector: 'node[status="PENDING"]',
              style: {
                'background-color': '#31CCEC' // Cyan for pending
              }
            },
            // SKIPPED - Grey
            {
              selector: 'node[status="SKIPPED"]',
              style: {
                'background-color': '#9AA0A6' // Grey for skipped
              }
            },
            // PAUSED - Grey with yellow border
            {
              selector: 'node[status="PAUSED"]',
              style: {
                'background-color': '#9AA0A6', // Grey
                'border-width': '3px',
                'border-color': '#F2C037' // Yellow border
              }
            },
            {
              selector: 'edge',
              style: {
                'width': 2,
                'line-color': '#9AA0A6',
                'target-arrow-color': '#9AA0A6',
                'target-arrow-shape': 'triangle',
                'curve-style': 'bezier',
                'transition-property': 'line-color, target-arrow-color, width',
                'transition-duration': '0.2s'
              }
            },
            {
              selector: 'edge.highlighted',
              style: {
                'width': 8, // Double thickness (from 4 to 8)
                'line-color': '#C10015', // Red highlight
                'target-arrow-color': '#C10015',
                'arrow-scale': 1.5, // Make arrows bigger too
                'z-index': 999 // Make highlighted edges appear on top
              }
            },
            {
              selector: 'node.selected',
              style: {
                'border-width': 6, // Thicker border for selected node
                'border-color': '#C10015',
                'border-style': 'double', // Double line style for emphasis

                // Directional shadow - light source from top left
                'shadow-blur': 25, // Large blur for diffuse shadow
                'shadow-color': '#000000', // Dark shadow
                'shadow-opacity': 0.6,
                'shadow-offset-x': 8, // Shadow to the right
                'shadow-offset-y': 8, // Shadow below

                // Red glow effect as a separate layer
                'underlay-color': '#ff0000',
                'underlay-opacity': 0.7,
                'underlay-padding': 10,
                'underlay-blur': 15,

                // Make text stand out
                'text-outline-color': '#222', // Ensure text remains readable
                'text-outline-width': 1.5,
                'color': '#fff',

                'z-index': 20, // Ensure selected node is on top
                'background-color': function(ele) {
                  // Keep original color but make it slightly brighter
                  return ele.style('background-color');
                }
              }
            },
            {
              selector: 'node.adjacent',
              style: {
                'border-width': 8, // Even thicker border
                'border-color': '#C10015',
                'border-style': 'solid',

                // Directional shadow - same light source as selected node (top left)
                'shadow-blur': 15, // Smaller blur than selected node
                'shadow-color': '#000000', // Dark shadow
                'shadow-opacity': 0.4, // Less opaque than selected
                'shadow-offset-x': 5, // Smaller offset than selected
                'shadow-offset-y': 5, // Smaller offset than selected

                // Red glow as separate layer
                'underlay-color': '#ff0000',
                'underlay-opacity': 0.5, // Less intense than selected
                'underlay-padding': 6,
                'underlay-blur': 10,

                // Text improvements
                'text-outline-color': '#222', // Make text readable over the glow
                'text-outline-width': 1,

                'z-index': 10 // Bring forward but below selected node
              }
            }
          ],
          layout: {
            name: 'dagre',
            rankDir: 'LR',
            nodeSep: 80,      // Increased node separation
            rankSep: 120,     // Increased rank separation
            animate: true,
            padding: 30       // Add padding
          }
        })

        // Apply our custom text wrapping
        wrapNodeLabels();

        // Add slight angle to the view for better shadow visibility
        cy.pan({ x: 0, y: 0 });
        cy.zoom(1);

        // Add click handler for nodes to highlight connections
        cy.on('tap', 'node', function(evt) {
          const node = evt.target;

          // Reset styles
          cy.elements().removeClass('highlighted adjacent');
          cy.nodes().removeClass('selected');

          // Highlight the selected node
          node.addClass('selected');

          // Find connected edges (both incoming and outgoing)
          const connectedEdges = node.connectedEdges();

          // Add highlight class to connected edges
          connectedEdges.addClass('highlighted');

          // Also highlight connected nodes
          const connectedNodes = node.neighborhood('node');
          connectedNodes.addClass('adjacent');
        });

        // Click on background to reset highlighting
        cy.on('tap', function(evt) {
          if (evt.target === cy) {
            // Reset all styles when clicking on background
            cy.elements().removeClass('highlighted adjacent');
            cy.nodes().removeClass('selected');
          }
        });

        // Fit the graph to the container
        cy.fit()
      }, 100)
    }

    onMounted(() => {
      fetchDagGraph()
    })

    onBeforeUnmount(() => {
      if (cy) {
        cy.destroy()
      }
    })

    // Filtered tasks based on search query and sorting
    const filteredTasks = computed(() => {
      if (!graph.value || !graph.value.tasks) return [];

      // Filter step: Apply search filter
      let filtered = graph.value.tasks;
      if (taskSearchQuery.value) {
        const searchTerm = taskSearchQuery.value.toLowerCase();
        filtered = filtered.filter(task => {
          // Search across all fields
          return (
            task.id.toLowerCase().includes(searchTerm) ||
            task.name.toLowerCase().includes(searchTerm) ||
            task.status.toLowerCase().includes(searchTerm) ||
            task.operator.toLowerCase().includes(searchTerm) ||
            (task.duration && task.duration.toString().includes(searchTerm))
          );
        });
      }

      // Sort step: Apply sorting if needed
      if (taskTablePagination.value.sortBy) {
        const { sortBy, descending } = taskTablePagination.value;
        const direction = descending ? -1 : 1;

        // Make a copy to avoid modifying the original array
        filtered = [...filtered].sort((a, b) => {
          // Handle special cases for date and duration
          if (sortBy === 'start_time' || sortBy === 'end_time') {
            // Sort dates
            const dateA = a[sortBy] ? new Date(a[sortBy]).getTime() : 0;
            const dateB = b[sortBy] ? new Date(b[sortBy]).getTime() : 0;
            return direction * (dateA - dateB);
          } else if (sortBy === 'duration') {
            // Sort durations with null values at the end
            const durA = a[sortBy] !== null ? a[sortBy] : Infinity;
            const durB = b[sortBy] !== null ? b[sortBy] : Infinity;
            return direction * (durA - durB);
          } else {
            // Default string/number sort
            const valA = a[sortBy] !== undefined ? a[sortBy] : '';
            const valB = b[sortBy] !== undefined ? b[sortBy] : '';

            // If string, use localeCompare, otherwise direct comparison
            if (typeof valA === 'string' && typeof valB === 'string') {
              return direction * valA.localeCompare(valB);
            } else {
              return direction * (valA > valB ? 1 : valA < valB ? -1 : 0);
            }
          }
        });
      }

      return filtered;
    });

    // Handle table sorting and pagination
    const onTaskTableRequest = (props) => {
      const { page, rowsPerPage, sortBy, descending } = props.pagination;

      // This triggers a re-sort in the computed property
      taskTablePagination.value = {
        ...taskTablePagination.value,
        page,
        rowsPerPage,
        sortBy,
        descending,
        rowsNumber: filteredTasks.value.length
      };

      // Update the table immediately to reflect changes
      console.log('Sort changed:', sortBy, descending ? 'DESC' : 'ASC');
    };

    // Update task filter
    const updateTaskFilter = (val) => {
      taskSearchQuery.value = val;
      // Reset to first page when filter changes
      taskTablePagination.value.page = 1;
    };

    // Format selection label
    const getSelectedString = () => {
      return selectedTask.value.length === 0
        ? ''
        : `${selectedTask.value.length} task${selectedTask.value.length > 1 ? 's' : ''} selected of ${filteredTasks.value.length}`;
    };

    // React to task selection in the table
    watch(selectedTask, (newVal) => {
      if (newVal.length > 0 && cy) {
        // Find the node in the graph that matches the selected task
        const taskId = newVal[0].id;
        const node = cy.getElementById(taskId);

        if (node.length > 0) {
          // Highlight the selected node
          cy.elements().removeClass('highlighted adjacent');
          cy.nodes().removeClass('selected');

          node.addClass('selected');

          // Highlight connected edges and nodes
          const connectedEdges = node.connectedEdges();
          connectedEdges.addClass('highlighted');

          const connectedNodes = node.neighborhood('node');
          connectedNodes.addClass('adjacent');

          // Center the view on the selected node
          cy.center(node);
        }
      }
    });

    return {
      dagId,
      graph,
      loading,
      error,
      taskColumns,
      getStatusColor,
      formatDate,
      formatDuration,
      filteredTasks,
      taskSearchQuery,
      taskTablePagination,
      selectedTask,
      onTaskTableRequest,
      updateTaskFilter,
      getSelectedString
    }
  }
}
</script>

<style>
.dag-cytoscape-container {
  height: 500px;
  width: 100%;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.perspective-container {
  perspective: 1000px;
  perspective-origin: center center;
}
</style>
