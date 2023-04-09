<template>
    <v-container>
        <v-card>
            <v-card-title>
                <h1 class="display-1">Add Funds</h1>
            </v-card-title>
            <v-select v-model="selectedMethod" :items="methods" label="Payment Method" outlined dense />
            <v-card-text>
                <v-if v-if="selectedMethod === 'Credit Card'">
                    <v-text-field v-model="cardNumber" label="Card Number" outlined dense />
                    <v-text-field v-model="cardHolder" label="Card Holder" outlined dense />
                    <v-text-field v-model="cardExpiration" label="Expiration Date" outlined dense />
                    <v-text-field v-model="cardCvv" label="CVV" outlined dense />
                    <v-text-field v-model="amount" label="Amount" outlined dense />
                </v-if>
                <v-if v-if="selectedMethod === 'Google Pay' || selectedMethod === 'Apple Pay'">
                    <v-text-field v-model="amount" label="Amount" outlined dense />
                </v-if>
            </v-card-text>
            <v-btn color="primary" block dark :disabled="!amount" @click="addFunds">
                Add Funds
            </v-btn>
        </v-card>
    </v-container>
</template>

<script>
export default{
    data(){
        return{
            selectedMethod: '',
            methods: ['Credit Card', 'Google Pay', 'Apple Pay'],
            cardNumber: '',
            cardHolder: '',
            cardExpiration: '',
            cardCvv: '',
            googlePay: '',
            applePay: '',
            amount: '',
        };
    },
    methods: {
        addFunds(){
            let user = JSON.parse(localStorage.getItem("user"));
            console.log(user)
            let new_amount = parseFloat(user.balance) + parseFloat(this.amount)
            console.log(
                JSON.stringify({
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    password: user.password,
                    balance: new_amount,
                })
            )
            localStorage.setItem("user", JSON.stringify({
                id: user.id,
                name: user.name,
                email: user.email,
                password: user.password,
                balance: new_amount,
            }))
            const response = this.$axios.$put('/api/user/' + user.id, JSON.stringify({
                id: user.id,
                name: user.name,
                email: user.email,
                password: user.password,
                balance: new_amount,
            }), {
                headers: {
                    'Content-Type': 'application/json',
                }
            }).then((response) => {
                console.log(response);
                this.$router.push('/profile');
            }).catch((error) => {
                console.log(error);
            });
        },
    },
}
</script>