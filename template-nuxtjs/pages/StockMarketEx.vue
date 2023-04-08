<template>
    <v-container>
        <!--Add header to explain-->
        <v-row class="text-center" justify="center">
            <v-col cols="12" sm="8" md="6">
                <h1>Stock Market Sentiment Analysis</h1>
                <p>Enter a stock ticker symbol to see the sentiment analysis of the last 100 tweets.</p>
            </v-col>
        </v-row>
        <v-row align="center" justify="center">
            <v-col cols="12" sm="8" md="6">
                <v-form @submit.prevent="sendMessage">
                    <v-text-field v-model="inputText" label="Enter Text" outlined
                        ></v-text-field>
                    <!-- Make the button full like the card -->
                    <v-btn color="primary" block type="submit">Show Results</v-btn>
                </v-form>

            </v-col>
        </v-row>

        <v-row justify="center">
            <v-col cols="12" sm="8" md="6">
                <h3>Progress Bars:</h3>
                <v-progress-linear :value="positivePercentage" height="35" color="green" rounded class="mb-3">
                    {{ positivePercentage }}%
                </v-progress-linear>

                <v-progress-linear :value="neutralPercentage" height="35" color="grey lighten-1" rounded class="mb-3">
                    {{ neutralPercentage }}%
                </v-progress-linear>

                <v-progress-linear :value="negativePercentage" height="35" color="red" rounded class="mb-3">
                    {{ negativePercentage }}%
                </v-progress-linear>
            </v-col>
        </v-row>
    </v-container>
</template>
  
<script>
export default {
    data() {
        return {
            inputText: "",
            positivePercentage: 0,
            negativePercentage: 0,
            neutralPercentage: 0,
        };
    },
    methods: {
        async queryData(data) {
            const response = await fetch(
                "https://api-inference.huggingface.co/models/ProsusAI/finbert",
                {
                    headers: { Authorization: "Bearer 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" },
                    method: "POST",
                    body: JSON.stringify(data),
                }
            );
            const result = await response.json();
            return result;
        },
        async sendMessage() {
            let responses_bot = null;
            await this.queryData({
                inputs: {
                    text: this.inputText
                }
            })
                .then(response => {
                    responses_bot = response;
                    response.forEach(element => {
                        if (element.label == "positive")
                            this.positivePercentage = element.score.toFixed(2) * 100;
                        else if (element.label == "negative")
                            this.negativePercentage = element.score.toFixed(2) * 100;
                        else if (element.label == "neutral")
                            this.neutralPercentage = element.score.toFixed(2) * 100;
                    });
                    console.log('Success:', response)
                })
        },
    },
};
</script>
  