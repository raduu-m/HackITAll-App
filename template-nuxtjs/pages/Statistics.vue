<style>
.horizontal-list {
    display: flex;
    overflow-x: auto;
    overflow-y: scroll;
    white-space: nowrap;
    float: left;
    display: inline;

}

.bar {

    width: 200px;
    height: 100vw;

    overflow-y: auto;

    transform-origin: top;

    transform: rotate(-90deg);

}

.mon {
    transform-origin: top;

    transform: rotate(90deg);
}
</style>

<template>
    <v-container>
        <v-card>
            <v-card-title>Statistics</v-card-title>
            <v-card-text>
                <v-row>
                    <v-col cols="4">
                        <v-progress-circular :value="percentage_sent" color="primary" size="100" width="10">
                            <div>{{ percentage_sent }}%</div>
                        </v-progress-circular>
                    </v-col>
                    <v-col cols="4">
                        <v-progress-circular :value="25" color="success" size="100" width="10">
                            <div>{{ 25 }}%</div>
                        </v-progress-circular>
                    </v-col>
                    <v-col cols="4">
                        <v-progress-circular :value="75" color="warning" size="100" width="10">
                            <div>{{ 75 }}%</div>
                        </v-progress-circular>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="4">
                        <v-chip color="primary">Category 1</v-chip>
                    </v-col>
                    <v-col cols="4">
                        <v-chip color="success">Category 2</v-chip>
                    </v-col>
                    <v-col cols="4">
                        <v-chip color="warning">Category 3</v-chip>
                    </v-col>
                </v-row>
                <v-divider></v-divider>
                <v-row>
                    <v-col cols="4">
                        <p>Total Transactions</p>
                        <h3>{{ total }}</h3>
                    </v-col>
                    <v-col cols="4">
                        <p>Total Transaction Recived</p>
                        <h3>{{ sum_received }}</h3>
                    </v-col>
                    <v-col cols="4">
                        <p>Total Transactions Given</p>
                        <h3>{{ sum_sent }}</h3>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
        <br><br><br><br><br>
        <v-card>
            <div class="horizontal-list bar">
                <v-list horizontal>
                    <v-list-item v-for="(exp, index) in monthly_expenses" :key="index">
                        <div class="mon">{{ months[index] }}</div>
                        <v-progress-linear class="bar_2" :value="exp"></v-progress-linear>
                    </v-list-item>
                </v-list>
            </div>
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
            this.sum_sent = 0;
            this.sum_received = 0;
            this.total = 0;
            this.percentage_sent = 0;

            for (var i = 1; i <= 12; i++) {
                this.monthly_expenses.push(0);
            }

            let max_month_sum = 0;

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
                    if (user.id == element.t1_id) {
                        title = "Payment Sent"
                        tid = element.t2_id
                        this.sum_sent += element.ammount;
                        this.monthly_expenses[month - 1] += element.ammount;
                    } else {
                        title = "Payment Received"
                        tid = element.t1_id
                        this.sum_received += element.ammount;
                        this.monthly_expenses[month - 1] += element.ammount;
                    }



                    this.transactions.push({ title: title, date: word_month + ' ' + day + ', ' + year, amount: element.ammount, id: tid })
                });

                this.total = this.sum_received + this.sum_sent;
                this.percentage_sent = (this.sum_sent * 100) / this.total;

                for (var i = 1; i <= 12; i++) {
                    this.months.push(monthMap.get(i).slice(0, 3))
                    if (max_month_sum < this.monthly_expenses[i]) {
                        max_month_sum = this.monthly_expenses[i]
                    }
                }

                console.log(this.months)

                for (var i = 0; i < 12; i++) {
                    this.monthly_expenses[i] = this.monthly_expenses[i] * 100 / max_month_sum
                }

            }).catch(error => {
                console.log(error);
            })
        }
    },
    data() {
        return {
            username: '',
            balance: 0.0,
            transactions: [
            ],
            percentage_sent: 0,
            sum_sent: 0,
            sum_received: 0,
            total: 0,
            monthly_expenses: [],
            months: [],

        }
    }
}
</script>
