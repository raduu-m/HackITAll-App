<template>
    <v-container>
        <v-card>
            <v-card-title>Statistics</v-card-title>
            <v-card-text>
                <v-row>
                    <v-col cols="4">
                        <v-progress-circular :value="50" color="primary" size="100" width="10">
                            <div>{{ 50 }}%</div>
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
                        <h3>5,000</h3>
                    </v-col>
                    <v-col cols="4">
                        <p>Total Transaction Recived</p>
                        <h3>2,500</h3>
                    </v-col>
                    <v-col cols="4">
                        <p>Total Transactions Given</p>
                        <h3>1,250</h3>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
        <div id="tester" style="width:600px;height:250px;"></div>
    </v-container>
</template>
  
<script>
var Plotly = require('plotly.js-dist-min')

export default{
    mounted(){
    }
}

</script>