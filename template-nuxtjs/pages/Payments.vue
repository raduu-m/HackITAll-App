<template>
  <v-container>
    <v-select v-model="selectedCategory" :items="categories" label="Payment Category" outlined dense />
    <v-divider class="my-4" />
    <v-card>
      <v-card-title class="text-h5">{{ selectedCategory }}</v-card-title>
      <v-card-text>
        <v-if v-if="selectedCategory === 'Restante' || selectedCategory === 'Campus'">
          <v-text-field v-model="amount" label="Amount due" outlined dense />
          <v-text-field v-if="selectedPaymentType === 'Student'" v-model="studentId" label="Student ID" outlined dense />
          <v-text-field v-if="selectedPaymentType === 'University'" v-model="paymentNote" label="Payment Note" outlined
            dense />
        </v-if>
        <v-if v-if="selectedCategory === 'Other Payments'">
          <v-select v-model="selectedPaymentType" :items="paymentTypes" label="Payment Type" outlined dense />
          <v-text-field v-model="amount" label="Amount due" outlined dense />
          <v-if v-if="selectedPaymentType === 'Student'">
            <v-text-field v-model="studentId" label="Student ID" outlined dense />
          </v-if>
          <v-if v-if="selectedPaymentType === 'University'">
            <v-text-field v-model="paymentNote" label="Payment Note" outlined dense />
          </v-if>
        </v-if>
        <v-btn color="primary" dark :disabled="!amount" @click="pay">
          Pay
        </v-btn>
      </v-card-text>
    </v-card>
    <v-divider class="my-4" />
    <v-card>
      <v-card-title class="text-h5">Last Transactions</v-card-title>
      <v-card-text>
        <!-- Display the last transactions -->
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script>
export default {
  data() {
    return {
      selectedCategory: '',
      categories: ['Restante', 'Campus', 'Other Payments'],
      selectedPaymentType: '',
      paymentTypes: ['Student', 'University'],
      amount: '',
      studentId: '',
      paymentNote: '',
    };
  },

  watch: {
    selectedCategory(newValue) {
      if (newValue === 'Restante' || newValue === 'Campus') {
        this.amount = '';
        this.studentId = '';
        this.paymentNote = '';
        this.selectedPaymentType = '';
      } else {
        this.amount = '';
        this.studentId = '';
        this.paymentNote = '';
        this.selectedPaymentType = '';
      }
    },
  },

  methods: {
    pay() {
      // Implement payment API call here
      const uid = JSON.parse(localStorage.getItem("user")).id;
      const user = JSON.parse(localStorage.getItem("user"));
      try {
        // send data as json
        const transaction_j = {
          id: "",
          t1_id: uid,
          t2_id: this.studentId,
          ammount: this.amount,
          timestamp: "2023020200"
        };

        if (this.selectedCategory === 'Restante') {
          localStorage.setItem('user', JSON.stringify({
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
            balance: user.balance - parseFloat(this.amount),
          }))
          this.$router.push('/')
        } else if (this.selectedCategory === 'Campus') {
          localStorage.setItem('user', JSON.stringify({
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
            balance: user.balance - parseFloat(this.amount),
          }))
          this.$router.push('/')
        } else if (this.selectedCategory === 'Other Payments') {
          if (this.selectedPaymentType === 'Student') {
            transaction_j.t2_id = this.studentId;
          } else if (this.selectedPaymentType === 'University') {
            localStorage.setItem('user', JSON.stringify({
              id: user.id,
              name: user.name,
              email: user.email,
              password: user.password,
              balance: user.balance - parseFloat(this.amount),
            }))
            this.$router.push('/')
          }
        }

        const response = this.$axios.$post('/api/transaction', JSON.stringify(
          {
            id: "",
            t1_id: uid,
            t2_id: this.studentId,
            ammount: parseFloat(this.amount),
            timestamp: "2023020200"
          }
        ), {
          headers: {
            'Content-Type': 'application/json'
          }
        })
        console.log(
          JSON.stringify(
            {
              transaction_j
            }
          )
        )
        // Store this in local storage
        // const arr = JSON.parse(localStorage.getItem('transactions')) || [];
        // arr.push(transaction_j);
        // localStorage.setItem(JSON.stringify('transactions', arr))

        this.$axios.$get('/api/user/' + uid).then((response) => {
          // Save the response to the store
          localStorage.setItem('user', JSON.stringify({
            id: response.id,
            name: response.name,
            email: response.email,
            password: response.password,
            balance: response.balance,
          })) // Redirect to the home page
          this.$router.push('/')
        }).catch(error => {
          console.log(error);
        })




        // Redirect to the home page
        this.$router.push('/')

      } catch (err) {
        console.log(err)
      }

    },
  },
};
</script>