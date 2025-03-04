<template>
  <div>
    <div v-for="dag in dags" :key="dag.id" class="q-mb-md">
      <dag-card
          :dag="dag"
          @toggle-paused="$emit('toggle-paused', $event)"
      />
    </div>
  </div>
</template>

<script>
import DagCard from './DagCard.vue'

export default {
  name: 'DagList',

  components: {
    DagCard
  },

  props: {
    dags: {
      type: Array,
      required: true
    }
  },

  emits: ['toggle-paused']
}
</script>