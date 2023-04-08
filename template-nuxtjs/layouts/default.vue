<template>
  <v-app dark style="background: linear-gradient(to bottom, #081B33, #0C3857);">
    <v-navigation-drawer
      v-model="drawer"
      :mini-variant="miniVariant"
      :clipped="clipped"
      fixed
      app
      style="background-color: rgba(8, 27, 51, 0.9);"
    >
      <v-list>
        <v-list-item
          v-for="(item, i) in items"
          :key="i"
          :to="item.to"
          router
          exact
          v-if="item.title !== 'logout'"
        >
          <v-list-item-action>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>{{ item.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item v-else @click="logout()">
          <v-list-item-action>
            <v-icon>mdi-logout</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Logout</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
      <v-btn
        @click="logout()"
        icon
        class="logout-button mx-2"
        style="position: absolute; bottom: 0; left: 30%; margin-bottom: 20px;"
      >
        <v-icon>mdi-logout</v-icon>
        Log out
      </v-btn>
    </v-navigation-drawer>

    <v-app-bar
      :clipped-left="clipped"
      fixed
      app
      style="background-color: rgba(8, 27, 51, 0.6);"
    >
      <v-app-bar-nav-icon @click.stop="drawer = !drawer" /> 
      <v-btn
        icon
        @click.stop="miniVariant = !miniVariant"
      >
        <v-icon>mdi-{{ `chevron-${miniVariant ? 'right' : 'left'}` }}</v-icon>
      </v-btn>
      <v-toolbar-title class="flex text-center" style="position: absolute; left: 50%; transform: translateX(-50%);">
        <img src="../assets/logo_cut.png" height="50" alt="Logo" style=" margin-top: 10px;" />
      </v-toolbar-title>
      <v-spacer />
    </v-app-bar>
    <v-main>
      <v-container>
        <Nuxt />
      </v-container>
    </v-main>
    <v-navigation-drawer
      v-model="rightDrawer"
      :right="right"
      temporary
      fixed
    >
      <v-list>
        <!-- <v-list-item @click.native="right = !right">
          <v-list-item-action>
            <v-icon light>
              mdi-repeat
            </v-icon>
          </v-list-item-action>
        </v-list-item> -->
      </v-list>
    </v-navigation-drawer>
    <v-footer
      :absolute="!fixed"
      app
    >
      <span>&copy; Segafault:Marxist</span>
    </v-footer>
  </v-app>
</template>



<script>
export default {
  name: 'DefaultLayout',
  data () {
    return {
      clipped: false,
      drawer: false,
      fixed: false,
      items: [
        {
          icon: 'mdi-home',
          title: 'Home',
          to: '/'
        },
        {
          icon: 'mdi mdi-account', //mdi-chart-bubble
          title: 'My Profile',
          to: '/profile'
        },
        {
          icon: 'mdi mdi-credit-card-outline', //mdi-chart-bubble
          title: 'Payments',
          to: '/payments'
        },
        {
          icon: 'mdi mdi-forum-outline', //mdi-chart-bubble
          title: 'Gheorghe',
          to: '/assistent'
        },
        {
          icon: 'mdi mdi-credit-card-outline', //mdi-chart-bubble
          title: 'Offers',
          to: '/offerts'
        },
        {
          icon: 'mdi mdi-shopping', //mdi-chart-bubble
          title: 'Marketplace',
          to: '/marketplace'
        },
      ],
      miniVariant: false,
      right: false,
      rightDrawer: false,
      title: 'StudentSpend'
    }
  },
  methods: {
    logout() {
      localStorage.clear();
      this.$router.push('/login');
      
    }
  }

}
</script>

<style>
  .v-navigation-drawer, .v-app-bar {
    
    color: white;
  }
  
  .v-list-item-title {
    font-weight: bold;
  }
</style>
