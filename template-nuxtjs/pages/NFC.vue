<template>
    <v-container>
        <v-row v-if="isMobile">
            <v-col>
                <v-container fill-height>
                <v-row align="center" justify="center">
                    <v-col cols="12" sm="8" md="4">
                        <v-sheet v-if="!isLoading" class="text-center">
                            <!--Button to trigger sendPay-->
                            <v-btn color="primary" dark @click="sendPay">Send</v-btn>
                            <p>Payment Sent</p>
                        </v-sheet>
                        <v-progress-circular v-else indeterminate size="64"></v-progress-circular>
                    </v-col>
                </v-row>
            </v-container>
            </v-col>
        </v-row>
        <v-row v-else>
            <!--This is not a mobile-->
            <p>This is not mobile</p>
        </v-row>
    </v-container>
</template>
  
<script>
export default {
    data() {
        return {
            isMobile: typeof navigator !== 'undefined' && /iPhone|iPad|iPod|Android/i.test(navigator.userAgent),
            isLoading: true
        }
    },
    mounted() {
        setTimeout(() => {
            this.isLoading = false
        }, 2000)
    },
    methods: {
        sendPay() {
            const user = JSON.parse(localStorage.getItem("user"));
            const uid = user.id;
            const transaction_j = {
                id: "",
                t1_id: uid,
                t2_id: 'uu',
                ammount: this.amount,
                timestamp: "2023020200"
            };
            const transaction = JSON.stringify(transaction_j);
            const response = this.$axios.$post('/api/transaction', JSON.stringify(
                {
                    id: "",
                    t1_id: uid,
                    t2_id: 'uu',
                    ammount: 100.0,
                    timestamp: "2023020200"
                }
            ), {
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            this.$axios.$get('/api/user/' + uid).then((response) => {
                // Save the response to the store
                localStorage.setItem('user', JSON.stringify({
                    id: response.id,
                    name: response.name,
                    email: response.email,
                    password: response.password,
                    balance: response.balance,
                }))
                this.isLoading = true;
                setTimeout(() => {
                    this.$router.push('/');
                }, 2000);
            }).catch(error => {
                console.log(error);
            })
        }
    }
}
</script>
  