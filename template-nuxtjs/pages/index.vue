<template>
  <v-container>
    <h1>Welcome, {{ username }}</h1>
    <v-card>
      <v-card-title>Balance (LEI)</v-card-title>
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
              <v-btn color="primary" text @click="viewTransaction(index)">View</v-btn>
            </v-list-item-action>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>
    <v-overlay v-if="activeIndex >= 0" :value="true">
      <v-card class="mx-auto text-center" style="width: 400px; height: 150px;">
        <v-card-title class="text-h5" style="" >{{ transactionDetails.title }}</v-card-title>
        <v-card-subtitle class="mb-3" style="position: absolute; left: 0;">{{ transactionDetails.date }}</v-card-subtitle>
        <v-card-text class="text-h6">{{ transactionDetails.description }}</v-card-text>
        <v-card-actions>
          <v-btn block color="primary" @click="deactivate()"  style="position: absolute; bottom: 0 padding: 20px">OK</v-btn>
        </v-card-actions>
      </v-card>
    </v-overlay>
  </v-container>
</template>
<script>
export default {
  name: 'Home',
  data() {
    return {
      username: 'John Doe',
      balance: 5000,
      transactions: [
        { title: 'Payment Received', date: 'April 1, 2023', description: 'You received a payment of $100.' },
        { title: 'Payment Sent', date: 'March 28, 2023', description: 'You sent a payment of $50.' },
        { title: 'Deposit', date: 'March 20, 2023', description: 'You deposited $1000.' },
        { title: 'Withdrawal', date: 'March 15, 2023', description: 'You withdrew $500.' },
        { title: 'Payment Received', date: 'March 10, 2023', description: 'You received a payment of $200.' }
      ],
      activeIndex: -1,
      transactionDetails: {}
    }
  },
  methods: {
    viewTransaction(index) {
      this.activeIndex = index;
      this.transactionDetails = this.transactions[index];
    },
    deactivate() {
      this.activeIndex = -1;
      this.transactionDetails = {};
    }
  }
}
</script> 