<template>
  <v-app>
    <v-card class="elevation-12" style=" height: 750px; max-width: 100%; ">
      <v-toolbar color="#495B99" dark>
        <v-toolbar-title>Gheorghe</v-toolbar-title>
        <v-spacer></v-spacer>
      </v-toolbar>
      <v-card-text style="height: 620px; overflow-y:auto;">
        <v-list>
          <v-list-item v-for="(message, index) in messages" :key="index">
            <v-list-item-content>
              <v-card :class="[message.sender === 'user' ? 'message-right' : 'message-left']" class="pa-3">
                <!-- <v-card-title v-if="message.sender === 'bot'" class="text-h6">Gheorghe:</v-card-title>
                <v-card-title v-else class="text-h6">You:</v-card-title> -->
                <v-card-text class="message-content">{{ message.content }}</v-card-text>
              </v-card>
            </v-list-item-content>
          </v-list-item>
        </v-list>
      </v-card-text>
      <div style="display: flex; align-items: center; position: absolute; bottom: 0; width: 95%; margin-left: 10px;">
        <v-form @submit.prevent="sendMessage" style="flex: 1;">
          <v-textarea
            v-model="messageContent"
            label="Type your message here"
            :rows="1"
            auto-grow
            clearable
            outlined
            dense
            required
          ></v-textarea>
        </v-form>
        <v-btn color="primary" type="submit" :disabled="!messageContent.trim()" style="margin-left: 16px; margin-bottom:25px" @click="sendMessage">Send</v-btn>
      </div>
    </v-card>
  </v-app>
</template>
  
<script>
import axios from 'axios'

export default {
  name: 'AIChatPage',
  data: () => ({
    messages: [],
    messageContent: ''
  }),
  methods: {
    async queryData(data) {
      const response = await fetch(
        "https://api-inference.huggingface.co/models/facebook/blenderbot-400M-distill",
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
      let past_user_inputs = []
      let generated_responses = []
      let responses_bot = null;

      // let output = await this.queryData({
      //   "inputs": {
      //     "past_user_inputs": [],
      //     "generated_responses": ["It's Die Hard for sure."],
      //     "text": "Can you explain why ?"
      //   }
      // }).then((response) => {
      //   console.log(JSON.stringify(response));
      // });
      if (!this.messageContent.trim()) {
        return
      }
      this.messages.push({
        sender: 'user',
        content: this.messageContent
      })
      past_user_inputs.push(this.messageContent)
      await this.queryData({
        inputs: {
          past_user_inputs: past_user_inputs,
          generated_responses: generated_responses,
          text: this.messageContent
        }
      })
        .then(response => {
          responses_bot = response;
          console.log('Success:', response.conversation.generated_responses[0]);
        })
        .catch(error => {
          console.error('Error:', error);
        });
      this.messages.push({
        sender: 'bot',
        content: responses_bot.conversation.generated_responses[0]
      })
      console.log(responses_bot)
      generated_responses.push(responses_bot.conversation.generated_responses[0])
      this.messageContent = ''
    },
    // async getResponse(message) {
    //   const postData = {
    //     message: message
    //   }
    //   return axios.post('/api/ai-chat', postData)
    // },
    logout() {
      localStorage.removeItem('token')
      this.$router.push('/login')
    }
  }
}
</script>

<style scoped>
.message-bubble {
  max-width:100%;
  font-size: 15;
  padding: 0.8rem;
  border-radius: 1rem;
  margin-bottom: 1rem;
}
.blue-grey {
  background-color: #B0BEC5 !important;
}
.message-right {
  margin-left: auto;
  background-color:#329940;
  max-width: 300px; 
  max-height: 150p
}

.message-left {
  margin-right: auto;
  background-color: #465199;
  max-width: 300px; 
  max-height: 150p
}

.message-content {
  font-size: 18px;
}
</style>
  
=======
  
<style scoped>
.blue-grey {
  background-color: #B0BEC5 !important;
}
</style>
