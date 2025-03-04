import { createApp } from 'vue'
import { Quasar } from 'quasar'
import quasarIconSet from 'quasar/icon-set/fontawesome-v5'

// Import Quasar CSS
import '@quasar/extras/roboto-font/roboto-font.css'
import '@quasar/extras/fontawesome-v5/fontawesome-v5.css'
import '@quasar/extras/material-icons/material-icons.css'
import 'quasar/dist/quasar.css'

// Import Font Awesome
import '@fortawesome/fontawesome-free/css/all.min.css'

// Import app CSS
import './css/app.scss'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(Quasar, {
    plugins: {}, // import Quasar plugins and add here
    iconSet: quasarIconSet
})

app.use(router)

// Mount the app
app.mount('#app')
