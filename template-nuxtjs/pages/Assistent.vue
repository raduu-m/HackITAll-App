<template>
    <v-app>
      <v-card class="elevation-12" style="; margin: 50px auto; height: 700px;  max-width: 100%;
      width: 800px;
      margin: 0 auto;">
        <v-toolbar color="primary" dark>
          <v-toolbar-title>AI Assistant Chat</v-toolbar-title>
          <v-spacer></v-spacer>
        </v-toolbar>
        <v-card-text style="height: 450px; overflow-y: scroll;">
          <v-list>
            <v-list-item v-for="(message, index) in messages" :key="index">
              <v-list-item-content>
                <v-card class="pa-2" :class="{ 'blue-grey': message.sender === 'bot', 'green lighten-3': message.sender === 'user' }">
                  <v-card-title v-if="message.sender === 'bot'" class="text-h6">AI Assistant:</v-card-title>
                  <v-card-title v-else class="text-h6">You:</v-card-title>
                  <v-card-text style="width: 100% margin: 50px auto">{{ message.content }}</v-card-text>
                </v-card>
              </v-list-item-content>
            </v-list-item>
          </v-list>
        </v-card-text>
        <v-card-actions>
          <v-form @submit.prevent="sendMessage">
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
            <v-btn color="primary" type="submit" :disabled="!messageContent.trim()">Send</v-btn>
          </v-form>
        </v-card-actions>
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
      async sendMessage () {
        if (!this.messageContent.trim()) {
          return
        }
        this.messages.push({
          sender: 'user',
          content: this.messageContent
        })
        const response = await this.getResponse(this.messageContent)
        this.messages.push({
          sender: 'bot',
          content: response.data.response
        })
        this.messageContent = ''
      },
      async getResponse (message) {
        const postData = {
          message: message
        }
        return axios.post('/api/ai-chat', postData)
      },
      logout () {
        localStorage.removeItem('token')
        this.$router.push('/login')
      }
    }
  }
  </script>
  
  <style scoped>
  .blue-grey {
    background-color: #B0BEC5 !important;
  }
  </style>
  