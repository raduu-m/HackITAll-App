<template>
  <v-container>
    <v-card class="mx-auto" max-width="400">
      <v-img
        :src="profilePic"
        height="200px"
        class="grey lighten-2"
      >
        <v-hover v-slot:default="{ hover }">
          <v-btn
            icon
            class="ma-2"
            color="white"
            v-if="hover"
            @click="changeProfilePic"
          >
            <v-icon>mdi-camera</v-icon>
          </v-btn>
        </v-hover>
      </v-img>
      <v-card-text>
        <v-text-field label="Name" v-model="name"></v-text-field>
        <v-text-field label="Account ID" v-model="accountId"></v-text-field>
        <v-text-field label="Balance" v-model="balance"></v-text-field>
      </v-card-text>
    </v-card>
  </v-container>
</template>
<script>
import QRCode from 'qrcode'

export default {
  data() {
    return {
      name: 'John Doe',
      accountId: '123456',
      balance: '$500',
      profilePic: ''
    }
  },
  methods: {
    changeProfilePic() {
      const data = `${this.name} (${this.accountId})`
      QRCode.toDataURL(data)
        .then(url => {
          this.profilePic = url
        })
        .catch(err => {
          console.error(err)
        })
    }
  }
}
</script>