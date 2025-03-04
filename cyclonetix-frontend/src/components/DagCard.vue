<template>
  <q-card class="airflow-card">
    <q-card-section>
      <div class="row justify-between items-start">
        <div>
          <div class="text-h6 text-weight-medium">
            <span class="q-mr-xs">
              <span
                  class="status-circle"
                  :class="{
                  'status-paused': dag.paused,
                  'status-running': !dag.paused && dag.running_count > 0,
                  'status-failed': !dag.paused && dag.failed_count > 0 && dag.running_count === 0,
                  'status-success': !dag.paused && dag.success_count > 0 && dag.failed_count === 0 && dag.running_count === 0
                }"
              ></span>
            </span>
            {{ dag.dag_id }}
          </div>
          <p class="q-mt-sm text-grey-7">
            {{ dag.description || 'No description' }}
          </p>
          <div class="q-mt-sm">
            <q-chip
                v-for="tag in dag.tags"
                :key="tag"
                size="sm"
                class="q-mr-xs"
                dense
            >
              {{ tag }}
            </q-chip>
          </div>
        </div>

        <div class="row q-gutter-sm">
          <q-btn
              :color="dag.paused ? 'positive' : 'warning'"
              :label="dag.paused ? 'Unpause' : 'Pause'"
              :icon="dag.paused ? 'fas fa-play' : 'fas fa-pause'"
              size="sm"
              flat
              @click="$emit('toggle-paused', { dagId: dag.dag_id, paused: !dag.paused })"
              no-caps
          />

          <q-btn
              color="primary"
              label="Trigger"
              icon="fas fa-play"
              size="sm"
              flat
              no-caps
          />
        </div>
      </div>

      <q-separator class="q-my-md" />

      <div class="row q-col-gutter-md">
        <div class="col-12 col-sm-4">
          <div class="text-grey-7 text-caption">Owner</div>
          <div class="text-body2 text-weight-medium">{{ dag.owner }}</div>
        </div>

        <div class="col-12 col-sm-4">
          <div class="text-grey-7 text-caption">Last Run</div>
          <div class="text-body2 text-weight-medium">{{ formatDate(dag.last_run) }}</div>
        </div>

        <div class="col-12 col-sm-4">
          <div class="text-grey-7 text-caption">Next Run</div>
          <div class="text-body2 text-weight-medium">{{ formatDate(dag.next_run) }}</div>
        </div>

        <div class="col-12 col-sm-4">
          <div class="text-grey-7 text-caption">Schedule</div>
          <div class="text-body2 text-weight-medium">{{ dag.schedule_interval }}</div>
        </div>

        <div class="col-12 col-sm-4">
          <div class="text-grey-7 text-caption">Runs</div>
          <div class="text-body2 text-weight-medium">
            <span class="q-mr-sm">Total: {{ dag.runs_count }}</span>
            <span class="text-positive q-mr-sm">Success: {{ dag.success_count }}</span>
            <span class="text-negative q-mr-sm">Failed: {{ dag.failed_count }}</span>
            <span v-if="dag.running_count > 0" class="text-info">
              Running: {{ dag.running_count }}
            </span>
          </div>
        </div>

        <div class="col-12 col-sm-4">
          <div class="text-grey-7 text-caption">Status</div>
          <div class="text-body2 text-weight-medium">
            <q-badge
                :color="getStatusColor()"
                text-color="white"
            >
              {{ getStatusText() }}
            </q-badge>
          </div>
        </div>
      </div>
    </q-card-section>

    <q-card-actions class="bg-grey-1 q-pa-sm justify-end">
      <q-btn 
        flat 
        color="primary" 
        label="View Graph" 
        icon="fas fa-eye" 
        no-caps 
        :to="{ name: 'dag-graph', params: { dagId: dag.dag_id }}"
      />
      <q-btn flat color="primary" label="View Code" icon="fas fa-code" no-caps />
      <q-btn flat color="primary" label="View History" icon="fas fa-history" no-caps />
    </q-card-actions>
  </q-card>
</template>

<script>
export default {
  name: 'DagCard',

  props: {
    dag: {
      type: Object,
      required: true
    }
  },

  emits: ['toggle-paused'],

  setup(props) {
    const formatDate = (dateString) => {
      if (!dateString) return 'N/A'

      const date = new Date(dateString)
      return date.toLocaleString()
    }

    const getStatusColor = () => {
      if (props.dag.paused) return 'warning'
      if (props.dag.running_count > 0) return 'info'
      if (props.dag.failed_count > 0) return 'negative'
      if (props.dag.success_count > 0) return 'positive'
      return 'grey'
    }

    const getStatusText = () => {
      if (props.dag.paused) return 'Paused'
      if (props.dag.running_count > 0) return 'Running'
      if (props.dag.failed_count > 0) return 'Failed'
      if (props.dag.success_count > 0) return 'Success'
      return 'No Status'
    }

    return {
      formatDate,
      getStatusColor,
      getStatusText
    }
  }
}
</script>