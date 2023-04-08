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
        <v-if v-if="selectedCategory === 'Restante' || selectedCategory === 'Campus'">
          <v-text-field
            v-model="amount"
            label="Amount due"
            outlined
            dense
          />
          <v-text-field
            v-if="selectedPaymentType === 'Student'"
            v-model="studentId"
            label="Student ID"
            outlined
            dense
          />
          <v-text-field
            v-if="selectedPaymentType === 'University'"
            v-model="paymentNote"
            label="Payment Note"
            outlined
            dense
          />
        </v-if>
        <v-if v-if="selectedCategory === 'Other Payments'">
          <v-select
            v-model="selectedPaymentType"
            :items="paymentTypes"
            label="Payment Type"
            outlined
            dense
          />
          <v-text-field
            v-model="amount"
            label="Amount due"
            outlined
            dense
          />
          <v-if v-if="selectedPaymentType === 'Student'">
            <v-text-field
              v-model="studentId"
              label="Student ID"
              outlined
              dense
            />
          </v-if>
          <v-if v-if="selectedPaymentType === 'University'">
            <v-text-field
              v-model="paymentNote"
              label="Payment Note"
              outlined
              dense
            />
          </v-if>
        </v-if>
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
      selectedPaymentType: '',
      paymentTypes: ['Student', 'University'],
      amount: '',
      studentId: '',
      paymentNote: '',
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
};
</script>
