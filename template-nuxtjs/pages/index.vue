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
              <v-list-item-title>{{ transaction.title }} - {{ transaction.id  }}</v-list-item-title>
              <v-list-item-subtitle>{{ transaction.amount  }}</v-list-item-subtitle>
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
  mounted() {
    // Get from the local storage
    const user = JSON.parse(localStorage.getItem('user'))
    if (user) {
      this.username = user.name
      this.balance = user.balance
    }
    else {
      this.$router.push('/login')
    }
    this.getTransactions()
  },
  methods: {
    getTransactions() {
      const monthMap = new Map([
        [1, 'January'],
        [2, 'February'],
        [3, 'March'],
        [4, 'April'],
        [5, 'May'],
        [6, 'June'],
        [7, 'July'],
        [8, 'August'],
        [9, 'September'],
        [10, 'October'],
        [11, 'November'],
        [12, 'December'],
      ]);

      let user = JSON.parse(localStorage.getItem('user'))
      this.$axios.$get('/api/user/transactions/' + JSON.parse(localStorage.getItem('user')).id).then((response) => {
        response.forEach(element => {
          console.log(element);
          let year = parseInt(element.timestamp.slice(0, 4))
          let month = element.timestamp.slice(4, 6)
          let day = parseInt(element.timestamp.slice(6, 9))
          let word_month = monthMap.get(parseInt(month))
          console.log(word_month)
          let title = null
          let tid = null
          if(user.id == element.t1_id){
            title = "Payment Sent"
            tid = element.t2_id
          }else{
            title = "Payment Received"
            tid = element.t1_id
          }
          this.transactions.push({title:title,date: word_month +' ' + day + ', ' + year, amount : element.ammount, id : tid})
        });
      }).catch(error => {
        console.log(error);
      })
    }
  },
  data() {



    return {
      username: '',
      balance: 0.0,
      transactions: [],
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