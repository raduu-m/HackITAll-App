<template>
    <v-container>
      <v-select
        v-model="selectedCategory"
        :items="categories"
        label="Payment Category"
        outlined
        dense
      />
      <v-divider class="my-4" />
      <v-card>
        <v-card-title class="text-h5">{{ selectedCategory }}</v-card-title>
        <v-card-text>
          <v-text-field
            v-if="selectedCategory === 'Restante' || selectedCategory === 'Campus'"
            v-model="amount"
            label="Amount due"
            outlined
            dense
          />
          <v-text-field
            v-if="selectedCategory === 'Other Payments'"
            v-model="amount"
            label="Amount due"
            outlined
            dense
          />
          <v-text-field
            v-if="selectedCategory === 'Other Payments'"
            v-model="studentId"
            label="Student ID"
            outlined
            dense
          />
          <v-btn
            color="primary"
            dark
            :disabled="!amount"
            @click="pay"
          >
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
        amount: '',
        studentId: '',
      };
    },
    methods: {
      pay() {
        // Implement payment API call here
      },
    },
    watch: {
      selectedCategory(newValue) {
        if (newValue === 'Restante' || newValue === 'Campus') {
          this.amount = '';
          this.studentId = '';
        } else {
          this.amount = '';
        }
      },
    },
  };
  </script>
  