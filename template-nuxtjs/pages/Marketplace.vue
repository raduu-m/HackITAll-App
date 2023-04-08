<template>
    <v-container>
      <v-row>
        <v-col md="6" lg="4" v-for="ad in ads" :key="ad.id">
          <v-card class="ad-card" :elevation="6">
            <v-img :src="ad.image" class="ad-image" />
            <v-card-title>{{ ad.title }}</v-card-title>
            <v-card-text>{{ ad.description }}</v-card-text>
            <v-card-actions>
              <v-btn text> {{ ad.price }} Lei</v-btn>
              <v-spacer />
              <v-btn color="primary" @click="buy(ad.id)">Buy</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
      <v-form @submit.prevent="postAd">
        <v-text-field v-model="title" label="Title" required></v-text-field>
        <v-textarea v-model="description" label="Description" required></v-textarea>
        <v-text-field v-model="price" label="Price" required></v-text-field>
        <v-file-input label="Image" v-model="image"></v-file-input>
        <v-btn type="submit" color="primary">Post Ad</v-btn>
      </v-form>
    </v-container>
  </template>
  
  <script>
  export default {
    data() {
      return {
        ads: [
          {
            id: 1,
            title: 'Vintage Camera',
            description: 'An old film camera in great condition',
            price: 120,
            image: 'https://via.placeholder.com/300x200/ebf8ff/000000?text=Vintage+Camera'
          },
          {
            id: 2,
            title: 'Laptop',
            description: 'A powerful laptop for gaming and work',
            price: 800,
            image: 'https://via.placeholder.com/300x200/ebf8ff/000000?text=Laptop'
          },
          {
            id: 3,
            title: 'Mountain Bike',
            description: 'A high-end mountain bike with full suspension',
            price: 1500,
            image: 'https://via.placeholder.com/300x200/ebf8ff/000000?text=Mountain+Bike'
          }
        ],
        title: '',
        description: '',
        price: '',
        image: null
      };
    },
    methods: {
      buy(adId) {
        alert('You bought ad #' + adId);
      },
      postAd() {
        const newAd = {
          id: this.ads.length + 1,
          title: this.title,
          description: this.description,
          price: this.price,
          image: null
        };
        if (this.image) {
          // Read the file as a base64 string and set it as the ad image
          const reader = new FileReader();
          reader.onload = () => {
            newAd.image = reader.result;
            this.ads.push(newAd);
          };
          reader.readAsDataURL(this.image);
        } else {
          this.ads.push(newAd);
        }
        this.title = '';
        this.description = '';
        this.price = '';
        this.image = null;
      }
    }
  };
  </script>
  
  