<template>
    <v-app>
      <v-content>
        <v-container fluid fill-height>
          <v-row align="center" justify="center">
            <v-col md="4">
              <v-card>
                <v-card-title class="text-center">Login</v-card-title>
                <v-card-text>
                  <v-form>
                    <v-text-field
                      v-model="email"
                      label="Email"
                      prepend-icon="mdi-account"
                      type="email"
                      required
                    ></v-text-field>
                    <v-text-field
                      v-model="password"
                      label="Password"
                      prepend-icon="mdi-lock"
                      type="password"
                      required
                    ></v-text-field>
                    <button type="submit" color="primary" >Login</button>
                  </v-form>
                </v-card-text>
                <v-card-actions>
              
                </v-card-actions>
              </v-card>
              <div>
                <p>Don't have an account? <nuxt-link to="/signup">Sign up</nuxt-link></p>
              </div>
            </v-col>
          </v-row>
        </v-container>
      </v-content>
    </v-app>
  </template>
  
  <script>
    import Vue from 'vue'
    import Vuetify from 'vuetify'
    // import 'vuetify/dist/vuetify.min.css'
  
    Vue.use(Vuetify)
  
    export default {
      layout: "no-layout",
      data() {
        return {
          email: '',
          password: '',
        }
      },
      
    }
  </script>
  