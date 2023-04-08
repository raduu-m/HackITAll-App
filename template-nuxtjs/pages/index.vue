<template>
  <v-container>
    <h1>Welcome, {{ username }}</h1>
    <v-card>
      <v-card-title>Balance</v-card-title>
      <v-card-text>
        <h2>{{ balance }}</h2>
      </v-card-text>
    </v-card>
    <v-card>
      <v-card-title>Transaction History</v-card-title>
      <v-card-text>
        <v-list>
          <v-list-item v-for="(transaction, index) in transactions" :key="index">
            <v-list-item-content>
              <v-list-item-title>{{ transaction.title }}</v-list-item-title>
              <v-list-item-subtitle>{{ transaction.date }}</v-list-item-subtitle>
            </v-list-item-content>
            <v-list-item-action>
              <v-btn color="primary" text>View</v-btn>
            </v-list-item-action>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script>
export default {
  name: 'Home',
  mounted() {
    // Get from the local storage
    const user = JSON.parse(localStorage.getItem('user'))
    if (user) {
      this.username = user.name
      this.balance = user.balance
    }
    else{
      this.$router.push('/login')
    }
  }, 
  data() {
    return {
      username: '',
      balance: 0.0,
      transactions: [
        { title: 'Payment Received', date: 'April 1, 2023' },
        { title: 'Payment Sent', date: 'March 28, 2023' },
        { title: 'Deposit', date: 'March 20, 2023' },
        { title: 'Withdrawal', date: 'March 15, 2023' },
        { title: 'Payment Received', date: 'March 10, 2023' }
      ]
    }
  }
}
</script>
