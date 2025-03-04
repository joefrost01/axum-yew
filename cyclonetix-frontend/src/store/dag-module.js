import { defineStore } from 'pinia'
import { api } from '../boot/axios.js'

export const useDagStore = defineStore('dags', {
    state: () => ({
        dags: [],
        totalCount: 0,
        loading: false,
        error: null,
        query: {
            page: 1,
            limit: 10,
            search: null,
            status: null,
            tags: null,
            sort_by: null,
            sort_order: null
        },
        currentDagGraph: null,
        graphLoading: false,
        graphError: null
    }),

    getters: {
        hasDags: (state) => state.dags.length > 0,
        hasNextPage: (state) => state.query.page * state.query.limit < state.totalCount,
        hasPreviousPage: (state) => state.query.page > 1,
        totalPages: (state) => Math.ceil(state.totalCount / state.query.limit)
    },

    actions: {
        async fetchDags() {
            this.loading = true
            this.error = null

            try {
                // Build query params
                const params = {}
                Object.entries(this.query).forEach(([key, value]) => {
                    if (value !== null && value !== undefined) {
                        params[key] = value
                    }
                })

                const response = await api.get('/dags', { params })
                this.dags = response.data.dags
                this.totalCount = response.data.total_count
            } catch (error) {
                console.error('Error fetching DAGs:', error)
                this.error = error.message || 'Error fetching DAGs'
            } finally {
                this.loading = false
            }
        },

        async fetchDagGraph(dagId) {
            this.graphLoading = true
            this.graphError = null

            try {
                const response = await api.get(`/dags/${dagId}/graph`)
                return response.data
            } catch (error) {
                console.error(`Error fetching DAG graph for ${dagId}:`, error)
                this.graphError = error.message || 'Error fetching DAG graph'
                throw error
            } finally {
                this.graphLoading = false
            }
        },

        async toggleDagPaused(dagId, paused) {
            try {
                // In a real app, we'd call the API here to toggle the paused state
                // await api.post(`/dags/${dagId}/paused`, { is_paused: paused })

                // For demo, update the local state directly
                const dag = this.dags.find(d => d.dag_id === dagId)
                if (dag) {
                    dag.paused = paused
                }
            } catch (error) {
                console.error(`Error toggling DAG ${dagId} paused state:`, error)
                throw error
            }
        },

        setPage(page) {
            this.query.page = page
            this.fetchDags()
        },

        setQuery(newQuery) {
            this.query = { ...this.query, ...newQuery, page: 1 } // Reset to page 1 when query changes
            this.fetchDags()
        }
    }
})
