import colors from 'vuetify/es5/util/colors'

export default {
  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    // titleTemplate: '%s - my-template2'
    title: 'Student Spend',
    htmlAttrs: {
      lang: 'en'
    },
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: '' },
      { name: 'format-detection', content: 'telephone=no' }
    ],
    link: [
      { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
    ]
  },

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: [
  ],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [
  ],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    // https://go.nuxtjs.dev/vuetify
    '@nuxtjs/vuetify',
  ],

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    '@nuxtjs/axios'
  ],

  axios: {
    proxyHeaders: false,
    credentials: false,
    proxy: true,
  },

  proxy:{
    '/api/': { target: 'http://localhost:8080', pathRewrite: {'^/api/': ''}, changeOrigin:true }
  },

  // Vuetify module configuration: https://go.nuxtjs.dev/config-vuetify
vuetify: {
    theme: {
      dark: true,
      themes: {
        dark: {
          primary: '#4caf50',
          secondary: {
            base: '#ff8c00',
            lighten3: '#ffb700',
            darken3: '#ff6200'
          },
          tertiary: {
            background: '#FFFFFF',
            surface: '#FFFFFF',
            primary: '#6200EE',
            'primary-darken-1': '#3700B3',
            secondary: '#03DAC6',
            'secondary-darken-1': '#018786',
            error: '#B00020',
            info: '#2196F3',
            success: '#4CAF50',
            warning: '#FB8C00',
          },
          accent: '#9c27b0'
        }
      }
    }
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
  },
  middleware: ['no-layout']

}
