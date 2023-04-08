<template>
  <v-app>
    <div class="logo-container">
      <img src="../assets/logo_transparent.png" height="140px" width="250px">
    </div>
    <v-content>
      <v-container fluid fill-height>
        <v-row align="center" justify="center" style="margin-top: -130px;">
          <v-col md="4">
            <v-card>
              <v-card-title class="text-center">Login</v-card-title>
              <v-card-text>
                <v-form @submit.prevent="submitForm" >
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
                  <!-- Make the button full like the card -->
                  <v-btn color="primary" block type="submit">Login</v-btn>
                </v-form>
              </v-card-text>
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
      methods:{
        submitForm(){
          console.log('/api/user/login/' + this.email + '/' + this.password);
          this.$axios.$post('/api/user/login/' + this.email + '/' + this.password).then((response) => {
              // Save the response to the store
              localStorage.setItem('user', JSON.stringify({
                id: response.id,
                name: response.name,
                email: response.email,
                password: response.password,
                balance: response.balance,
              }))
              // Redirect to the home page
              this.$router.push('/')
            }).catch(error => {
              console.log(error);
            })
        }
      }
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