<template>
    <v-app>
      <div class="logo-container">
        <img src="../assets/logo_transparent.png" height="140px" width="250px">
      </div>
      <v-content>
        <v-container fluid fill-height>
          <v-row align="center" justify="center" style="margin-top: -70px;">
            <v-col md="4">
              <v-card>
                <v-card-title class="text-center">Sign Up</v-card-title>
                <v-card-text>
                  <v-form>
                    <v-text-field
                      v-model="name"
                      label="Name"
                      prepend-icon="mdi-account"
                      type="name"
                      required
                    ></v-text-field>
                    <v-text-field
                      v-model="email"
                      label="Email"
                      prepend-icon="mdi-email"
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
                  </v-form>
                </v-card-text>
                <v-card-actions>
                  <v-btn color="primary" block>Login</v-btn>
                </v-card-actions>
              </v-card>
              <div>
                <p>Already have an account? <nuxt-link to="/login">Log in</nuxt-link></p>
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

  <style>
  .logo-container {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 50px;
  }
  </style>